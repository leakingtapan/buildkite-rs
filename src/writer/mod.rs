use reqwest;
use serde::Serialize;
use serde_json;

pub fn print_json<T: Serialize>(res: &Result<T, reqwest::Error>) {
    match res {
        Ok(r) => {
            let output = serde_json::to_string(r).unwrap();
            print!("{}", output);
        }
        Err(e) => print!("{:#?}", e),
    };
}
