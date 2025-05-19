//use rocket::Request;
use rocket_dyn_templates::{Template, context};
use rocket::response::status::NotFound;
use rocket::Request;

pub fn error404(e: &str) -> NotFound<Template> {
    NotFound( Template::render("base", context!{ error: e }) )
}

#[catch(404)]
pub fn not_found(req: &Request) -> NotFound<Template> {
    let e = format!("'{}' is not a valid path.", req.uri());
    error404( e.as_str() )
}