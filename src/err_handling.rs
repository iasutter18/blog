//use rocket::Request;
use rocket_dyn_templates::{Template, context};
use rocket::response::status::NotFound;

pub fn error404(e: &str) -> NotFound<Template> {
    NotFound( Template::render("base", context!{ error: e }) )
}