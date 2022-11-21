use authetications::auth_server::{Auth, AuthServer};
use authetications::{
    AuthServiceRequest, AuthServiceResponse, AuthenticationAnswerRequest,
    AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse,
    RegisterRequest, RegisterResponse,
};
use lib::{calculate_q, generate_prime, generate_random, generators_g};
use num_primes::{BigUint, Generator};
use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

pub mod authetications {
    tonic::include_proto!("authetications");
}

pub mod lib;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AuthService {
    pub p: BigUint,                                      //prime number  p
    pub q: BigUint,                                      // q = p - 1 / 2
    pub g: Vec<BigUint>,                                 // generators g
    pub i: BigUint,                                      // i = g^a mod p
    pub h: BigUint,                                      // h = g^b mod p
    pub h_i: BigUint,                                    // h_i = g^ab mod p
    pub users: HashMap<String, User>, // users hashmap key = user_id, value = User
    pub authentication: HashMap<String, Authentication>, // authentication hashmap key = auth_id, value = Authentication
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct User {
    pub user_id: String, // user id
    pub y1: BigUint,     // y1 = g^x mod p
    pub y2: BigUint,     // y2 = h^x mod p
}

// #[derive(Debug, Default, Clone, PartialEq, Eq)]
// pub struct Vector<User> {
//     pub users: Vec<User>,
// }

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Authentication {
    pub user: String,    // user id
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

        //self.users.insert(user_id.clone(), new_registration);
        println!("==========REGISTER==========");
        println!("self: {:?}", &self);

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

        let mut authentication: HashMap<String, Authentication> = self.authentication.clone();
        authentication.insert(auth_id.clone(), new_authentication);

        println!("authentication: {:?}", authentication);

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

        // let test = self.authentication.get_key_value(&auth_id);
        println!("test: {:?}", &self);

        let reply: AuthenticationAnswerResponse = AuthenticationAnswerResponse {
            session_id: format!("s: {},  auth_id: {}", req.s, &auth_id).into(),
        };

        return Ok(Response::new(reply));
    }
}

impl User {
    fn new(user_id: String, y1: BigUint, y2: BigUint) -> User {
        return User { user_id, y1, y2 };
    }
}

impl Authentication {
    fn new(user: String, r1: BigUint, r2: BigUint, auth_id: String, c: BigUint) -> Self {
        return Authentication {
            user,
            r1,
            r2,
            auth_id,
            c,
        };
    }
}

// impl AuthService {
//     fn insert_user(&mut self, user: User) {
//         self.users.insert(user.user.clone(), user);
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prime: BigUint = generate_prime(10).unwrap();
    println!("p: {}", prime);
    let q: BigUint = calculate_q(&prime).unwrap();
    println!("q: {}", q);
    let a: BigUint = Generator::new_composite(10);
    println!("a: {}", a);
    let b: BigUint = Generator::new_composite(10);
    println!("b: {}", b);
    let g: Vec<BigUint> = generators_g(&prime);
    println!("g: {:?}", g);

    // A = g^a mod p
    let i: BigUint = g[1].modpow(&a, &prime);
    println!("A: {}", i);

    // B = g^b mod p
    let h: BigUint = g[1].modpow(&b, &prime);
    println!("h: {}", h);

    // C = g^ab mod p
    let h_i: BigUint = g[1].modpow(&(&a * &b), &prime);
    println!("h_i: {}", h_i);

    let addr = "[::1]:50051".parse()?;
    let mut auth_service: AuthService = AuthService::default();

    auth_service.p = prime;
    auth_service.q = q;
    auth_service.g = g;
    auth_service.i = i;
    auth_service.h = h;
    auth_service.h_i = h_i;

    println!("auth_service: {:?}", auth_service);

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;
    Ok(())
}
