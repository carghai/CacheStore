pub mod routes;

use crate::database_func;
use rocket::*;

pub struct Web {}

pub struct StateData {
    api_key: String,
    null: String,
}

impl Web {
    pub fn start(&self) {
        let state = StateData {
            api_key: "your_api_key".to_string(), //TODO when using this please write an api key
            null: "null_nil_value_key:345n,234lj52".to_string(),
        };
        tokio::spawn(async {
            database_func::Func {}.example().await;
            // use this if you want to spawn aync funtions to modify values in state
        });
        ignite()
            .manage(state)
            .mount(
                "/",
                routes![
                    routes::index,
                    routes::display_data::read,
                    routes::add_data::add,
                    routes::delete::delete,
                    routes::null_write::null_write,
                ],
            )
            .launch();
    }
}
