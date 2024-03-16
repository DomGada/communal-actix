

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

mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}


mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;
    #[derive(Deserialize, PostgresMapper,Serialize)]
    #[pg_mapper(table="users")] 
    pub struct ComReq {
        pub reqType: String,
    
    }
    pub struct User{
        pub email: String,
        pub username: String,
        pub description: String,
        pub skills: Vec<String>,
    
    }

}
mod handlers{
    use actix_web::{web, App, HttpServer, Result};
    use crate::models::ComReq;
    pub async fn eventFilter(req: web::Json<ComReq>) -> Result<String> {

        if  req.reqType == "poopy"{
            Ok(poopyEvent(req).await?)
        }
        else if req.reqType == "ADD_USER"{
            
        }
        else if req.reqType == "UPDATE_USER"{
            
        }
        else{
            Ok(format!("Wow!"))
        }

}

    //pub async fn updateUser(req: web::Json<UserReq>)->Result<String>{

    //}

    pub async fn poopyEvent (req: web::Json<ComReq>) -> Result<String>{
        Ok(format!("You are in poopy! {}", req.reqType))
    }
}
use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;
use crate::handlers::index;
#[actix_web::main]
async fn main()-> std::io::Result<()>{

    HttpServer::new(||  App::new().route("/", web::post().to(eventFilter)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await


}

