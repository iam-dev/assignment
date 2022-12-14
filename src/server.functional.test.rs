use authetications::auth_server::{Auth, AuthServer};
use authetications::{
    AuthServiceRequest, AuthServiceResponse, AuthenticationAnswerRequest,
    AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse,
    RegisterRequest, RegisterResponse,
};
use lib::{calculate_q, generators_g};
use num_primes::BigUint;
use num_traits::Pow;
use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

pub mod authetications {
    tonic::include_proto!("authetications");
}

pub mod lib;

use std::sync::{Arc, Mutex};

type Users = Arc<Mutex<HashMap<String, User>>>;
type Authentications = Arc<Mutex<HashMap<String, Authentication>>>;

#[derive(Debug)]
pub struct AuthService {
    pub p: BigUint,                   //prime number  p
    pub q: BigUint,                   // q = p - 1 / 2
    pub g: Vec<BigUint>,              // generators g
    pub h: BigUint,                   // h = g^b mod p
    users: Users,                     // users Mutex
    authentications: Authentications, // authentications Mutex
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
    // Process: Setup steps
    // Name: AuthService
    // Description: return setup steps values
    // Inputs: None
    // Return: part of AuthService struct
    async fn auth_service(
        &self,
        request: Request<AuthServiceRequest>,
    ) -> Result<Response<AuthServiceResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = AuthServiceResponse {
            p: format!("{}", self.p.clone()).into(),
            q: format!("{}", self.q.clone()).into(),
            g: format!("{}", self.g[0].clone()).into(),
            h: format!("{}", self.h.clone()).into(),
        };
        return Ok(Response::new(reply));
    }

    // Process: Registration process
    // Name: register user
    // Inputs: RegisterRequest
    // - string: user name
    // - string: public value y1
    // - string: public value y2
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

    // Process: Login process
    // Name: Create Authentication Challenge
    // Inputs: RegisterRequest
    // - string: user id
    // - string: public r1
    // - string: public r2
    // Returns:
    // - string: uuid auth_id
    // - string: random generated c
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

        println!("Step 3.3) Generate random c");
        // Generate random c;
        //let c: BigUint = generate_random(10).unwrap();
        let c: BigUint = BigUint::from(17u32);
        println!("c = {}", &c);

        let new_challenge: Authentication = Authentication::new(
            req.user,
            req.r1.parse::<BigUint>().unwrap(),
            req.r2.parse::<BigUint>().unwrap(),
            auth_id.clone(),
            c.clone(),
        );

        let mut new_hashmap: HashMap<String, Authentication> = HashMap::new();
        new_hashmap.insert(auth_id.clone(), new_challenge);

        let new_authentication = Arc::new(Mutex::new(new_hashmap));
        println!("new_authentication: {:?}", new_authentication);

        {
            let mut authetications = self.authentications.lock().unwrap();
            authetications.extend(new_authentication.lock().unwrap().clone());
        }

        println!("self authentications: {:?}", self.authentications);

        let reply: AuthenticationChallengeResponse = AuthenticationChallengeResponse {
            auth_id: auth_id.clone(),
            c: format!("{}", c.clone()).into(),
        };

        return Ok(Response::new(reply));
    }

    // Process: Login process
    // Name: Verify Authentication
    // Inputs: RegisterRequest
    // - string: uuid auth_id
    // - string: computed s
    // Returns:
    // - string: uuid session_id
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
        let authentications: Authentications = self.authentications.clone();
        let authentication: Authentication = authentications
            .lock()
            .unwrap()
            .get(auth_id.as_str())
            .unwrap()
            .clone();
        println!("authentication: {:?}", authentication);

        let users: Users = self.users.clone();
        let user: User = users
            .lock()
            .unwrap()
            .get(authentication.user_id.as_str())
            .unwrap()
            .clone();
        println!("user: {:?}", user);

        let p: BigUint = self.p.clone();
        println!("p: {}", &p);

        let g: BigUint = self.g[0].clone();
        println!("g: {}", &g);

        let h: BigUint = self.h.clone();
        println!("h: {}", &h);

        let y1: BigUint = user.y1.clone();
        println!("y1: {}", &y1);
        let y2: BigUint = user.y2.clone();
        println!("y2: {}", &y2);

        let r1: BigUint = authentication.r1.clone();
        println!("r1: {}", r1);
        let r2: BigUint = authentication.r2.clone();
        println!("r2: {}", r2);

        let c2: BigUint = authentication.c.clone();
        println!("c: {}", c2);

        let s: BigUint = req.s.parse::<BigUint>().unwrap();
        println!("s: {}", s);

        // 1) verify g^s * y1^c  mod p == r1
        let verify_one: BigUint = g.pow(&s) * y1.pow(&c2) % &p;
        println!("verify_one: {}", verify_one);

        // 2) verify h^s * y1^c  mod p == r2
        let verify_two: BigUint = h.pow(&s) * y2.pow(&c2) % &p;
        println!("verify_two: {}", verify_two);

        if verify_one == r1 && verify_two == r2 {
            let session_id: String = Uuid::new_v4().to_string();

            let reply: AuthenticationAnswerResponse = AuthenticationAnswerResponse {
                session_id: format!("s: {}", session_id).into(),
            };
            return Ok(Response::new(reply));
        } else {
            return Err(Status::permission_denied("Could not verify authentication"));
        }
    }
}

// Setup steps
impl Default for AuthService {
    fn default() -> Self {
        //1) Setup steps
        println!("1) Setup steps");
        println!("Step 1.1) Generate Prime number p");
        // let prime: BigUint = generate_prime(10).unwrap();
        let prime: BigUint = BigUint::from(31u32);
        println!("p: {}", prime);

        println!("Step 1.2) Calculate q, such that p = 2q + 1");
        let q: BigUint = calculate_q(&prime).unwrap();
        println!("q: {}", q);

        println!("Step 1.3) Generate random b");
        //let b: BigUint = Generator::new_composite(10);
        let b: BigUint = BigUint::from(2u32);
        println!("b: {}", b);

        println!("Step 1.4) Generate generators, such that g is a generator of Zp*");
        let generators: Vec<BigUint> = generators_g(&prime);
        println!("generators: {:?}", generators);

        // Let's choose generator g = 4
        // g[0] = 4
        let g: BigUint = generators[0].clone();

        println!("Step 1.5) Compute h");
        // h = g^b mod p
        let h: BigUint = g.modpow(&b, &prime);
        println!("h = g^b mod p = {}", h);

        return AuthService {
            p: prime,
            q,
            g: generators,
            h,
            users: Arc::new(Mutex::new(HashMap::new())),
            authentications: Arc::new(Mutex::new(HashMap::new())),
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let auth_service: AuthService = AuthService::default();

    println!("auth_service: {:?}", auth_service);

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;
    Ok(())
}
