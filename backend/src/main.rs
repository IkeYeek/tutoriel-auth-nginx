use std::io;
use std::io::Error;
use actix_web::{error, get, http::{header::ContentType, StatusCode}, App, HttpResponse, HttpServer, HttpRequest, Responder};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "forbidden")]
    Forbidden,
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::Forbidden => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

#[get("/")]
async fn test(req: HttpRequest) -> impl Responder {
    println!("=== / ===");
    println!("{req:?}");
    "Hello World"
}

#[get("/auth")]
async fn auth(req: HttpRequest) -> Result<&'static str, MyError> {
    println!("=== /auth ===");
    println!("{req:?}");
    Err(MyError::Forbidden)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(test).service(auth)
    ).bind(("0.0.0.0", 8080))?.run().await
}