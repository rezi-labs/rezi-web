use actix_web::{ web, Scope};
use actix_files as fs;


pub fn scope()-> Scope{
    web::scope("").service(  fs::Files::new("/assets", "assets")
                .show_files_listing()
                .use_last_modified(true),)
}
