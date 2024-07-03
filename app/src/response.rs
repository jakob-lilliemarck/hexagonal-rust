use hexagonal::DrivingError;
use poem_openapi::payload::Json;
use poem_openapi::types::ParseFromJSON;
use poem_openapi::types::ToJSON;
use poem_openapi::ApiResponse;
use std::fmt::Debug;

#[derive(ApiResponse)]
pub enum MyResponse<T: 'static + ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<T>),
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalServerError,
}

type HandlerResult<T> = Result<Option<T>, DrivingError>;

impl<T: Debug + ParseFromJSON + ToJSON + Send + Sync> From<HandlerResult<T>> for MyResponse<T> {
    fn from(result: Result<Option<T>, DrivingError>) -> MyResponse<T> {
        match result {
            Ok(Some(entity)) => MyResponse::Ok(Json(entity)),
            Ok(None) => MyResponse::NoContent,
            Err(e) => match e {
                DrivingError::BadRequest => MyResponse::BadRequest,
                DrivingError::DrivenError(_) => MyResponse::NotFound,
                _ => MyResponse::InternalServerError,
            },
        }
    }
}
