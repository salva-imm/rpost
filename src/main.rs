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

use argon2::{self, Config};

use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;


#[crud_table]
#[derive(Clone, Debug)]
pub struct Users {
    pub id: String,
    pub name: String,
    pub password: String
}

#[derive(Deserialize, Validate)]
pub struct UserInput {
    pub id: String,
    pub name: String,
    pub password: String
}

pub const POSTGRES_URL: &'static str = "postgres://postgres:postgres@localhost:5434/postgres";

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref ARGON2_CONFIG: Config<'static> = Config::default();
    static ref ARGON2_SALT: &'static [u8]= "randomsalt".as_bytes();
}

#[post("/users/")]
async fn index(user: Json<UserInput>) -> impl Responder {
    let user_model = Users {
        id: user.id.to_owned(),
        name: user.name.to_owned(),
        password: argon2::hash_encoded(user.password.to_owned().as_bytes(), &ARGON2_SALT, &ARGON2_CONFIG).unwrap()
    };
    println!("{:#?}", user_model);
    let result =  RB.save(&user_model).await;
    HttpResponse::Ok().body(format!(
        "Welcome {:#?}, user_id {:#?}!, result {:#?}",
        user.id, user.name, result
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