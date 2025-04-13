use rocket_sync_db_pools::rusqlite::{self, params, Error};
use crate::DbConn;
use serde::Serialize;
use markdown::{to_html_with_options, CompileOptions, Options};
use rocket_dyn_templates::{Template, context};
use rocket::response::status::NotFound;
use crate::err_handling::error404;

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Default)]
pub struct Page {
    slug: String,  
    html: String,
    markdown: String,
    title: String
}

/*
* TODO: Write tests where appropriate.
*/
impl Page {
    pub async fn load_by_slug(slug: String, db: DbConn) -> Result<Template, NotFound<Template>> {
        let page = Self::load_from_db(slug.to_string(), db).await;
        match page {
            Ok(page) => Ok( Template::render("base", context!{ page }) ),
            Err(e) => Err( error404( &e.to_string() ) )
        }
    }
    async fn load_from_db(slug: String, db: DbConn) -> Result<Self, Error> {
        let q = "select * from pages where slug = ?1";
        db.run(move 
            |c| c.query_row(
                q,
                params![slug],
                Self::parse_db_row
        ))
        .await
    }
    fn parse_db_row(r: &rusqlite::Row<'_>) -> Result<Page, Error> {
        Ok( Page {
            slug: r.get("slug")?,
            html: r.get("html")?,
            markdown: r.get("markdown")?,
            title: r.get("title")?
        }.parse_markdown() )
        
    }
    fn parse_markdown(mut self) -> Self {
        match !self.html.is_empty() {
            true => self,
            false => { 
                self.html = to_html_with_options(&self.markdown, &Options { 
                    compile: CompileOptions {
                        allow_dangerous_html: true,
                        ..CompileOptions::default()
                    },
                    ..Options::default()
                }).unwrap() /* unwrap() here is safe without MDX enabled */; 
                self 
            }
        }
    }
}   