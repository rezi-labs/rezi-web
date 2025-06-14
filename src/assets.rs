use actix_files as fs;
use actix_web::{Scope, web};

pub fn scope() -> Scope {
    web::scope("").service(
        fs::Files::new("/assets", "assets")
            .show_files_listing()
            .use_last_modified(true),
    )
}
