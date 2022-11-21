extern crate postgres;

use authetications::auth_server::{Auth, AuthServer};
use authetications::{
    AuthServiceRequest, AuthServiceResponse, AuthenticationAnswerRequest,
    AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse,
    RegisterRequest, RegisterResponse,
};
use lib::{calculate_q, generate_prime, generate_random, generators_g};
use num_primes::{BigUint, Generator};
use postgres::Client;
use std::collections::HashMap;
use std::fs;
use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

pub mod authetications {
    tonic::include_proto!("authetications");
}

pub mod lib;

const INIT_SQL: &str = "./schema.sql";

use std::sync::{Arc, Mutex};

type Users = Arc<Mutex<HashMap<String, User>>>;

#[derive(Debug)]
pub struct AuthService {
    pub p: BigUint,      //prime number  p
    pub q: BigUint,      // q = p - 1 / 2
    pub g: Vec<BigUint>, // generators g
    pub i: BigUint,      // i = g^a mod p
    pub h: BigUint,      // h = g^b mod p
    pub h_i: BigUint,    // h_i = g^ab mod p
    users: Users,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct User {
    pub user_id: String, // user id
    pub y1: BigUint,     // y1 = g^x mod p
    pub y2: BigUint,     // y2 = h^x mod p
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Authentication {
    pub user_id: String, // user id
    pub r1: BigUint,     // r1 = g^k mod p
    pub r2: BigUint,     // r2 = h^k mod p
    pub auth_id: String, // random string
    c: BigUint,          // random BigUint
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn auth_service(
        &self,
        request: Request<AuthServiceRequest>,
    ) -> Result<Response<AuthServiceResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = AuthServiceResponse {
            p: format!("{}", self.p.clone()).into(),
            q: format!("{}", self.q.clone()).into(),
            g: format!("{}", self.g[1].clone()).into(),
            i: format!("{}", self.i.clone()).into(),
            h: format!("{}", self.h.clone()).into(),
            h_i: format!("{}", self.h_i.clone()).into(),
        };
        return Ok(Response::new(reply));
    }

    // Name: Registration process
    // Inputs: RegisterRequest
    // - string: user name
    // - int64: public value y1
    // - int64: public value y2
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req: RegisterRequest = request.into_inner();

        let user_id: String = req.user;

        let new_registration: User = User::new(
            user_id.clone(),
            req.y1.parse::<BigUint>().unwrap(),
            req.y2.parse::<BigUint>().unwrap(),
        );

        let mut new_hashmap: HashMap<String, User> = HashMap::new();
        new_hashmap.insert(user_id.clone(), new_registration);

        let new_users = Arc::new(Mutex::new(new_hashmap));
        println!("new_users: {:?}", new_users);

        {
            let mut users = self.users.lock().unwrap();
            users.extend(new_users.lock().unwrap().clone());
        }

        println!("self users: {:?}", self.users);

        let reply: RegisterResponse = RegisterResponse {};

        return Ok(Response::new(reply));
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!(
            "create_authentication_challenge: Got a request: {:?}",
            request
        );
        let req: AuthenticationChallengeRequest = request.into_inner();
        println!("create_authentication_challenge: {:?}", req);

        let auth_id: String = Uuid::new_v4().to_string();

        // Generate random c;
        let c: BigUint = generate_random(10).unwrap();
        println!("c = {}", &c);

        let new_authentication: Authentication = Authentication::new(
            req.user,
            req.r1.parse::<BigUint>().unwrap(),
            req.r2.parse::<BigUint>().unwrap(),
            auth_id.clone(),
            c.clone(),
        );

        // self.db.execute(
        //     "INSERT INTO users (auth_id, r1, r2, c) VALUES ($1, $2, $3, $4) WHERE user_id = $5",
        //     &[
        //         &auth_id,
        //         &new_authentication.r1,
        //         &new_authentication.r2,
        //         &c,
        //         &req.user,
        //     ],
        // )?;

        let reply: AuthenticationChallengeResponse = AuthenticationChallengeResponse {
            auth_id: auth_id.clone(),
            c: format!("{}", c.clone()).into(),
        };

        return Ok(Response::new(reply));
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        // Get user by user ID:

        println!("verify_authentication: Got a request: {:?}", request);

        let req: AuthenticationAnswerRequest = request.into_inner();
        let auth_id: String = req.auth_id;
        println!("verify_authentication: auth_id: {}", &auth_id);

        println!("==========verify_authentication==========");
        // let authentication: &Authentication = self.get_authentication(auth_id).unwrap();
        // println!("test: {:?}", &authentication);

        let reply: AuthenticationAnswerResponse = AuthenticationAnswerResponse {
            session_id: format!("s: {}", req.s).into(),
        };

        return Ok(Response::new(reply));
    }
}

impl Default for AuthService {
    fn default() -> Self {
        //1) Setup steps
        println!("1) Setup steps");
        println!("Step 1.1) Generate Prime number p");
        let prime: BigUint = generate_prime(10).unwrap();
        println!("p: {}", prime);

        println!("Step 1.2) Calculate q, such that p = 2q + 1");
        let q: BigUint = calculate_q(&prime).unwrap();
        println!("q: {}", q);

        println!("Step 1.3) Generate random a and b");
        let a: BigUint = Generator::new_composite(10);
        println!("a: {}", a);
        let b: BigUint = Generator::new_composite(10);
        println!("b: {}", b);

        println!("Step 1.4) Generate generators, such that g is a generator of Zp*");
        let g: Vec<BigUint> = generators_g(&prime);
        println!("g: {:?}", g);

        println!("Step 1.5) Calculate h, i, h_i");

        // A = g^a mod p
        let i: BigUint = g[1].modpow(&a, &prime);
        println!("A = g^a mod p = {}", i);

        // h = g^b mod p
        let h: BigUint = g[1].modpow(&b, &prime);
        println!("h = g^b mod p = {}", h);

        // h_i = g^ab mod p
        let h_i: BigUint = g[1].modpow(&(&a * &b), &prime);
        println!("h_i = g^ab mod p = {}", h_i);

        return AuthService {
            p: prime,
            q,
            g,
            i,
            h,
            h_i,
            users: Arc::new(Mutex::new(HashMap::new())),
        };
    }
}

impl User {
    fn new(user_id: String, y1: BigUint, y2: BigUint) -> User {
        return User { user_id, y1, y2 };
    }
}

impl Authentication {
    fn new(user_id: String, r1: BigUint, r2: BigUint, auth_id: String, c: BigUint) -> Self {
        return Authentication {
            user_id,
            r1,
            r2,
            auth_id,
            c,
        };
    }
}

// impl AuthService {
//     fn get_user(&self, user_id: String) -> Option<&User> {
//         return self.users.get(&user_id);
//     }

//     fn get_authentication(&self, auth_id: String) -> Option<&Authentication> {
//         return self.authentication.get(&auth_id);
//     }

//     fn add_user(&mut self, user: User) {
//         self.users.insert(user.user_id.clone(), user);
//     }

//     fn add_authentication(&mut self, authentication: &Authentication) {
//         self.authentication
//             .insert(authentication.auth_id.clone(), authentication.clone());
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mut auth_service: AuthService = AuthService::default();

    println!("auth_service: {:?}", auth_service);

    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let init_file = fs::read_to_string(INIT_SQL)?;

    // Execute the query.
    client
        .batch_execute(&init_file)
        .await
        .expect("Error executing schema.sql");

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;
    Ok(())
}
