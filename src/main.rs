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
use err_handling::not_found;

#[launch]
async fn start() -> _ {
    rocket::build()
        .mount("/", routes![index, toplevel_pages, preview_toplevel_pages])       
        .mount("/static", FileServer::from("./static")) 
        .register("/", catchers![not_found])
        //.attach(Template::fairing())
        .attach(DbConn::fairing())
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper( "is_active", Box::new(is_active) );
        }))
}