use rocket::response::status::NotFound;
use rocket_dyn_templates::Template;
use crate::DbConn;

use crate::pages::Page;

#[get("/")]
pub async fn index(db: DbConn) -> Result<Template,NotFound<Template>> {
    Page::load_by_slug("index".into(), db).await
}

#[get("/<slug>")]
pub async fn toplevel_pages(db: DbConn, slug: &str) -> Result<Template,NotFound<Template>> {
    Page::load_by_slug(slug.into(), db).await
}