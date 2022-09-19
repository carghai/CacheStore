#![cfg(test)]

use crate::func::http_request;
use crate::StateData;
use core::time;
use std::thread::{self};

static BASE_URL: &str = "http://localhost:8000";
// http://0.0.0.0:8000

async fn test_url(url: String) {
    let test_data = http_request::Request::read(url).await;
    match test_data {
        Err(e) => {
            panic!("Make Sure Main Code is running\n\n {}", e)
        }
        Ok(data) => {
            if data.error != "Success" {
                panic!("{}", data.error);
            }
        }
    }
}
#[tokio::test]
async fn add_data_test() {
    test_url(
        format!(
            "{}/add/test`worked/data/this is going very well/your_api_key",
            BASE_URL
        )
        .to_owned(),
    )
    .await;
    test_url(
        format!(
            "{}/add/test`worked/data/this is going very well/your_api_key",
            BASE_URL
        )
        .to_owned(),
    )
    .await;
}
#[tokio::test]
async fn read_data_test() {
    thread::sleep(time::Duration::from_secs(1));
    test_url(format!("{}/read/test`worked`data/your_api_key", BASE_URL).to_owned()).await;
}
#[tokio::test]
async fn read_data_test_many() {
    thread::sleep(time::Duration::from_secs(1));
    let data = http_request::Request::read_more(
        format!("{}/read/test`worked`data/your_api_key", BASE_URL).to_owned(),
    )
    .await
    .expect("");
    if data.data
        != vec![
            "this is going very well".to_owned(),
            "this is going very well".to_owned(),
        ]
    {
        panic!("data does not equal correct amount")
    }
}
#[tokio::test]
async fn delete_data_test() {
    thread::sleep(time::Duration::from_secs(3));
    test_url(format!("{}/delete/test`worked`data/your_api_key", BASE_URL).to_owned()).await;
}
#[tokio::test]
async fn delete_data_test_check() {
    thread::sleep(time::Duration::from_secs(4));
    let data = http_request::Request::read(
        format!("{}/read/test`worked`data/your_api_key", BASE_URL).to_owned(),
    )
    .await
    .expect("");
    if data.error == "Success" {
        panic!("data was not deleted")
    }
}
#[tokio::test]
async fn null_test() {
    thread::sleep(time::Duration::from_secs(5));
    test_url(format!("{}/null_write/test`worked`data/your_api_key", BASE_URL).to_owned()).await;
}
#[tokio::test]
async fn null_test_check() {
    thread::sleep(time::Duration::from_secs(6));
    test_url(
        format!(
            "{}/add/test`worked/data/this is going very well/your_api_key",
            BASE_URL
        )
        .to_owned(),
    )
    .await;
}
#[tokio::test]
async fn read_data_test_null() {
    thread::sleep(time::Duration::from_secs(7));
    test_url(format!("{}/read/test`worked`data/your_api_key", BASE_URL).to_owned()).await;
    test_url(format!("{}/delete/test`worked`data/your_api_key", BASE_URL).to_owned()).await;
}
// Local functions testing
//_________________________________________________________________________________________________________________________
fn test_read_error(func: StateData, what_error: String) {
    let check_data = func.read_data("test/worked/local/data");
    match check_data {
        Ok(data) => {
            panic!("{}", data[0].to_owned())
        }
        Err(e) => {
            if e != what_error {
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn add_data_test_local() {
    let func = StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    };
    func.write_data("this is going very well", "test/worked/local", "data")
        .expect("failed when writing data");
    func.write_data("this is going very well", "test/worked/local", "data")
        .expect("failed when writing data");
}
#[test]
fn read_data_test_local() {
    thread::sleep(time::Duration::from_secs(1));
    let func = StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    };
    let check_data = func
        .read_data("test/worked/local/data")
        .expect("failed when reading");
    if check_data
        != vec![
            "this is going very well".to_owned(),
            "this is going very well".to_owned(),
        ]
    {
        panic!("write data did not work!")
    }
}
#[test]
fn null_test_check_local() {
    thread::sleep(time::Duration::from_secs(2));
    let func = StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    };
    func.null_write("test/worked/local/data")
        .expect("write null functions failed");
    func.write_data("test/worked/local", "test/worked/local", "data")
        .expect("msg");
    test_read_error(func, "data is null".to_owned());
}
#[test]
fn delete_data_test_check_local() {
    thread::sleep(time::Duration::from_secs(3));
    let func = StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    };
    func.delete_data("test/worked/local/data")
        .expect("delete failed");
    test_read_error(func, "No such file or directory (os error 2)".to_owned());
}
