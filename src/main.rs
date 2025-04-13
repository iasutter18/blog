#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use rocket_sync_db_pools::{database, rusqlite};
#[database("blog")]
#[allow(dead_code)]
struct DbConn(rusqlite::Connection);

mod routes; 
use routes::*;
mod pages;
mod err_handling;

#[launch]
async fn start() -> _ {
    rocket::build()
        .mount("/", routes![index, toplevel_pages])       
        .mount("/static", FileServer::from("./static")) 
        .attach(Template::fairing())
        .attach(DbConn::fairing())
}
 