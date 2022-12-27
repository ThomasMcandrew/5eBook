use crate::models::login::LoginModel;
use crate::api::*;

pub async fn login(model: LoginModel) -> Result<LoginModel, Error>
{
    post::<LoginModel>(
            String::from("user/login"),
            model
        )
        .await
}
pub async fn create_user(model: LoginModel) -> Result<LoginModel, Error>
{
    post::<LoginModel>(
            String::from("user"),
            model
        )
        .await
}
