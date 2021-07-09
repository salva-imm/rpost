#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use actix_web::{
    get, post, web, App, HttpResponse,
    HttpServer, Responder, middleware};
use serde::Deserialize;
use actix_web_validator::Json;
use validator::Validate;

#[allow(unused_imports)]
use argon2::{self, Config};

use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;


#[crud_table]
#[derive(Clone, Debug)]
pub struct Users {
    pub id: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>
}

impl Default for Users {
    fn default() -> Self {
        Users {
            id: None,
            name: None,
            password: None
        }
    }
}



#[derive(Deserialize, Validate)]
struct Info {
    user_id: u32,
    friend: String,
}

pub const POSTGRES_URL: &'static str = "postgres://postgres:postgres@localhost:5434/postgres";

// init global rbatis pool
lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[post("/users/")]
async fn index(info: Json<Info>) -> impl Responder {
    let users = Users {
        id: Some(info.user_id.to_string()),
        name: Some(info.friend.to_string()),
        password: None
    };
    println!("{:#?}", users);
    let result =  RB.save(&users).await;
    HttpResponse::Ok().body(format!(
        "Welcome {}, user_id {}!, result {:#?}",
        info.friend, info.user_id, result
    ))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo/")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _request_logs = fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    //link database
    RB.link(POSTGRES_URL).await.unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(hello)
            .service(echo)
            .service(index)
            .route("/hey/", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}