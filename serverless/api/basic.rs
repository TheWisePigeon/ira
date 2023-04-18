use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use cola_auth::{respond, astra_db_config};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let test = astra_db_config();
    println!("{:?}", test);
    respond(StatusCode::OK)
}
