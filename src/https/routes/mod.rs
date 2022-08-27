pub mod display_data;
pub mod add_data;
pub mod delete;

use rocket::*;


#[get("/")]
pub fn index() -> &'static str {
    "Welcome to RustStore!\n\
    The current routes are \n\
    [/add/<path>/<data_name>/<data>]\n\
    [/delete/<path>]\n\
    [/read/<path>]"
}



