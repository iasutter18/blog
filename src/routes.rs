use rocket::response::status::NotFound;
use rocket_dyn_templates::{Template, handlebars};
use crate::DbConn;


use crate::pages::Page;

#[get("/")]
pub async fn index(db: DbConn) -> Result<Template,NotFound<Template>> {
    Page::load_by_slug_optkey("index".into(), None, db).await
}

#[get("/<slug>")]
pub async fn toplevel_pages(db: DbConn, slug: &str) -> Result<Template, NotFound<Template>> {
    Page::load_by_slug_optkey(slug.into(), None, db).await
}

#[get("/preview/<slug>/<key>")]
pub async fn preview_toplevel_pages(db: DbConn, slug: &str, key: &str) -> Result<Template, NotFound<Template>> {
    Page::load_by_slug_optkey(slug.into(), Some(key.into()), db).await
}

pub fn is_active(
    h: &handlebars::Helper<'_>,
    _: &handlebars::Handlebars,
    c: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    o: &mut dyn handlebars::Output
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        let slug = c.data()["page"]["slug"].as_str().unwrap_or("");
        if slug == param.value().as_str().unwrap_or("oops") {
            o.write("class=\"active\"")?;
        }
    }else {
        o.write("oops")?;
    }

    Ok(())
}