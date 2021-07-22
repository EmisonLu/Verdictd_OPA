use std::fs::File;
use std::io::prelude::*;
use serde_json::{Result, Value};
use std::error::Error;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let s1 = "test.rego";
    let s2 = r#"
            {
                "mrEnclave" : "123",
                "mrSigner" : "456",
                "productId" : "789"
            }
            "#;
    let result = set_reference(s1, s2);
}

// fn get_str_from_value(value: Value) -> &str {

// }

pub fn set_reference(policy_name : &str, references : &str) -> bool {

    let references: Value = match serde_json::from_str(references) {
        Ok(res) => res,
        Err(_) => {
            println!("JSON unmashall failed");
            return false
        },
    };

    let mrEnclave: String = String::from("mrEnclave = ") + &references["mrEnclave"].to_string();
    let mrSigner: String = String::from("mrSigner = ") + &references["mrSigner"].to_string();
    let productId: String = String::from("productId = ") + &references["productId"].to_string();
    
    let policy = "package demo\n\n".to_owned() + &mrEnclave + "\n" + &mrSigner + "\n" + &productId + "\n
default allow = false

allow = true {
    mrEnclave == input.mrEnclave
    mrSigner == input.mrSigner
    productId == input.productId
}
    ";

    let path = String::from("src/policy/") + policy_name;
    let path = Path::new(&path);
    let display = path.display();

    // Open the file in write-only mode, return `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => {
            panic!("couldn't create {}: {}", display, why.description());
            return false;
        },
        Ok(file) => file,
    };

    // Write the string `policy` into `file`, return `io::Result<()>`
    match file.write_all(policy.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, why.description());
            return false;
        },
        Ok(_) => (),
    }

    true
}