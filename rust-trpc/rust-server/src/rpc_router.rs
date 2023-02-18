use crate::data_holder;

use tonic::{Request, Response, Status};
pub use user_register::user_register_server::{UserRegister,UserRegisterServer};
use user_register::{RegisterUserRequest, RegisterUserReply,UsersReply,Empty};

pub mod user_register {
    tonic::include_proto!("user_register");
}

#[derive(Debug, Default)]
pub struct MyUserRegister {}

#[tonic::async_trait]
impl UserRegister for MyUserRegister {
    
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserReply>, Status> {

        println!("Got a request: {:?}", request);
        let user = request.into_inner().name.clone();
        data_holder::register_user(user.clone());


        let reply = user_register::RegisterUserReply {
            message: format!("Hello {}!", &user),
        };

        Ok(Response::new(reply))
    }

    async fn load_all_users(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<UsersReply>, Status> {

        println!("Got a request: {:?}", request);

        let data = data_holder::get_users();

        let reply = user_register::UsersReply {
            users: data,
        };

        Ok(Response::new(reply))
    }

}