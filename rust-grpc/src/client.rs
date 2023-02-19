use tonic::transport::Channel;
use user_register::user_register_client::UserRegisterClient;
use user_register::{RegisterUserRequest,Empty};

pub mod user_register {
    tonic::include_proto!("user_register");
}


async fn register_user(mut client:UserRegisterClient<Channel>,name:String) -> Result<(), Box<dyn std::error::Error>>{

    let request = tonic::Request::new(RegisterUserRequest {
        name: name.into(),
    });

    let response = client.register_user(request).await?;

    println!("RESPONSE={:?}", response);



    Ok(())
}

async fn get_users(mut client:UserRegisterClient<Channel>) -> Result<(), Box<dyn std::error::Error>>{

    let request = tonic::Request::new(Empty {});

    let response = client.load_all_users(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UserRegisterClient::connect("http://[::1]:50051").await?;
    
    register_user(client.clone(),"idir".to_string()).await?;
    get_users(client.clone()).await?;
    
    Ok(())

}
