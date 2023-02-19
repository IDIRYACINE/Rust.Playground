mod data_holder;
mod rpc_router;

use rpc_router::{MyUserRegister, UserRegisterServer};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let greeter =  MyUserRegister::default();

    Server::builder()
        .add_service(UserRegisterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
