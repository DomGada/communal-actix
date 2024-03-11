use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;


// Here is our actix web server
// Here we will create API endpoints for serving our postgres shit.
// TODO LIST
// 1.) How do I break up rust into subfiles?
// 2.) How do I import said sub files?
// 3.) What are some best practices for Actix?
//
//
//
// In rust std::io::Result<()> indicates the result of an operation succeeding.
// For exmaple in the past we have past Ok(()) as a valid return reponse for the above.
//
// format! is a commonly used method that concatinates strings together ie: format!("Welcome",
// username)
//
//
//https://github.com/actix/examples/blob/master/databases/postgres/src/main.rs   How to use
//postgres and actix

#[derive(Deserialize)]
struct ComReq {
    reqType: String,
    
}
mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;
    #[derive(Deserialize, PostgresMapper,Serialize)]
    #[pg_mapper(table="users")] 

    pub struct User{
        pub email: String,
        pub username: String,
        pub description: String,

    
    }

}


mod handlers{

async fn index(req: web::Json<ComReq>) -> Result<String> {

    if  req.reqType == "poopy"{
        Ok(poopyEvent(req).await?)
    }
    else{
        Ok(format!("Wow!"))
    }

}

async fn updateUser(req: web::Json<UserReq>)->Result<String>{

}

async fn poopyEvent (req: web::Json<ComReq>) -> Result<String>{
    Ok(format!("You are in poopy! {}", req.reqType))
}
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{

    HttpServer::new(||  App::new().route("/", web::post().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await


}

