#![feature(proc_macro_hygiene, decl_macro)]

use std::fmt::Display;

mod func;
mod https;

/// rustup override set nightly
/// do this so nightly can be used in the code
/// this code will only work with nightly

pub struct StateData {
    pub api_key: String,
    pub null: String,
    pub data_storage_location: String,
}

impl StateData {
    fn error_or_not(&self, error: String) -> Result<(), String> {
        if error == "Success" {
            Ok(())
        } else {
            Err(error)
        }
    }
    fn path_format(&self, path: String) -> String {
        return format!("{}{}", self.data_storage_location, path);
    }

    pub fn start_database_online(self) {
        https::Web {}.start(self);
    }
    pub fn write_data<T: Display, T2: Display, T3: Display>(
        &self,
        data: T,
        path: T2,
        data_name: T3,
    ) -> Result<(), String> {
        let final_path = self.path_format(path.to_string());
        let read_data = https::routes::add_data::AddDataFunc {}.core(
            final_path,
            data_name.to_string(),
            data.to_string(),
            self.api_key.to_string(),
        );
        return self.error_or_not(read_data.error);
    }
    pub fn delete_data<T: Display>(&self, path: T) -> Result<(), String> {
        let final_path = self.path_format(path.to_string());
        let delete_error = https::routes::delete::DeleteFunc {}.main_func(final_path);
        return self.error_or_not(delete_error.error);
    }
    pub fn read_data<T: Display>(&self, path: T) -> Result<Vec<String>, String> {
        let final_path = self.path_format(path.to_string());
        let read_error =
            https::routes::display_data::DisplayFunc {}.core(final_path, self.null.to_string());
        if read_error.error == "Success" {
            Ok(read_error.data)
        } else {
            Err(read_error.error)
        }
    }
    pub fn null_write<T: Display>(&self, path: T) -> Result<(), String> {
        let final_path = self.path_format(path.to_string());
        let null_error =
            https::routes::null_write::NullFunc {}.core(self.null.to_string(), final_path);
        return self.error_or_not(null_error.error);
    }
}
