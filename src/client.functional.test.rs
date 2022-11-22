use authetications::auth_client::AuthClient;
use authetications::{
    AuthServiceRequest, AuthServiceResponse, AuthenticationAnswerRequest,
    AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse,
    RegisterRequest, RegisterResponse,
};
use num_primes::BigUint;
use tonic::{Request, Response};
use uuid::Uuid;

pub mod authetications {
    tonic::include_proto!("authetications");
}

pub mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_uuid: String = Uuid::new_v4().to_string();

    let mut client = AuthClient::connect("http://[::1]:50051").await?;

    let request: Request<AuthServiceRequest> = tonic::Request::new(AuthServiceRequest {});

    let response: Response<AuthServiceResponse> = client.auth_service(request).await?;

    println!("RESPONSE={:?}", &response);

    // Get the public values <p, q, g, i, h, h_i> from the server
    let p_temp: u128 = (response.get_ref().p.clone()).parse().unwrap();
    println!("p_temp = {}", p_temp);
    let p: BigUint = BigUint::from(p_temp);
    println!("p = {}", p);

    let q_temp: u128 = (response.get_ref().q.clone()).parse().unwrap();
    println!("q_temp = {}", q_temp);
    let q: BigUint = BigUint::from(q_temp);
    println!("q = {}", q);

    let g_temp: u128 = (response.get_ref().g.clone()).parse().unwrap();
    println!("g_temp = {}", g_temp);
    let g: BigUint = BigUint::from(g_temp);
    println!("g = {}", g);

    let h_temp: u128 = (response.get_ref().h.clone()).parse().unwrap();
    println!("h_temp = {}", h_temp);
    let h: BigUint = BigUint::from(h_temp);
    println!("h = {}", h);

    println!("Step 2.1) Choose secret number x");
    // Generate a secret number x
    //let x: BigUint = generate_random(10).unwrap();
    let x: BigUint = BigUint::from(624u32);
    println!("x = {}", x);

    println!("Step 2.2) Compute y1 and y2");
    // Compute y1 and y2
    //y1 = g^x mod p
    let y1: BigUint = g.modpow(&x, &p);
    println!("y1 = {}^{} mod {} = {}", g, x, p, y1);

    // y2 = h^x mod p
    let y2: BigUint = h.modpow(&x, &p);
    println!("y2 = {}^{} mod {} = {}", h, x, p, y2);

    // Register
    let register_request: Request<RegisterRequest> = tonic::Request::new(RegisterRequest {
        user: user_uuid.clone(),
        y1: format!("{}", &y1).into(),
        y2: format!("{}", &y2).into(),
    });
    let register_response: Response<RegisterResponse> = client.register(register_request).await?;

    println!("Register RESPONSE={:?}", &register_response);

    println!("3) Login process");
    println!("Step 3.1) Random k");
    // Generate commitment C
    // Generate random number k
    //let k: BigUint = generate_random(20).unwrap();
    let k: BigUint = BigUint::from(492u32);
    println!("k = {}", k);

    println!("Step 3.2) Compute r1 and r2");
    //r1 = g^k mod p
    let r1: BigUint = g.modpow(&k, &p);
    println!("r1 = {}", r1);

    // r2 = h^k mod p
    let r2: BigUint = h.modpow(&k, &p);
    println!("r2 = {}", r2);

    //create_authentication_challenge
    let authentication_challenge_request: Request<AuthenticationChallengeRequest> =
        tonic::Request::new(AuthenticationChallengeRequest {
            user: user_uuid.clone(),
            r1: format!("{}", &r1).into(),
            r2: format!("{}", &r2).into(),
        });

    let authentication_challenge_response: Response<AuthenticationChallengeResponse> = client
        .create_authentication_challenge(authentication_challenge_request)
        .await?;
    println!(
        "AuthenticationChallenge RESPONSE={:?}",
        &authentication_challenge_response
    );

    let c: BigUint = (authentication_challenge_response.get_ref().c.clone())
        .parse::<BigUint>()
        .unwrap();
    println!("c = {}", c);

    let auth_id: String = authentication_challenge_response.get_ref().auth_id.clone();
    println!("auth_id = {}", auth_id);

    println!("Step 3.4) Compute s");
    // compute s
    // s = k - c * x mod q
    let s: BigUint = &k - &c * &x % &q;
    println!("s = {}", s);

    let verify_authentication_request: Request<AuthenticationAnswerRequest> =
        tonic::Request::new(AuthenticationAnswerRequest {
            auth_id: auth_id.clone(),
            s: format!("{}", &s).into(),
        });

    let authentication_answer_response: Response<AuthenticationAnswerResponse> = client
        .verify_authentication(verify_authentication_request)
        .await?;

    println!("RESPONSE={:?}", &authentication_answer_response);

    Ok(())
}
