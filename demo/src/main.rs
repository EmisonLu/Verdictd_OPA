use std::fs::File;
use std::io::prelude::*;
use serde_json::{Result, Value};
use std::error::Error;
use std::path::Path;

pub type GoUint8 = ::std::os::raw::c_uchar;
pub struct GoString {
    pub p: *const ::std::os::raw::c_char,
    pub n: isize,
}

#[link(name = "double_input")]
extern {
    // fn DoubleInput(input: libc::c_int) -> libc::c_int;
    fn Hello();
    pub fn handleReference(mr_ref: GoString);
    pub fn handleInput(mr_usr: GoString) -> GoUint8;
}

fn main() {
    // let c_str = CString::new("{\"MRENCLAVE\":\"73284f63a6d8796f\",\"MRSIGNER\":\"c4219b312ce36827\"}").unwrap();
    let c_str = "{\"MRENCLAVE\":\"73284f63a6d8796f\",\"MRSIGNER\":\"c4219b312ce36827\"}".to_string();
    let h = c_str.as_ptr() as *const i8;
    let input = GoString {
        p: h,
        n: c_str.len() as isize,
    };
    unsafe {handleReference(input);}
    // let input = 2;
    // let output = unsafe { DoubleInput(input) };
    // println!("{} * 2 = {}", input, output);
    unsafe {Hello()};
    // unsafe{World()};
    let c_str1 = "{\"MRENCLAVE\":\"73284f63a6d8796f\",\"MRSIGNER\":\"c4219b312ce36827\"}".to_string();
    let h1 = c_str1.as_ptr() as *const i8;
    let input1 = GoString {
        p: h1,
        n: c_str1.len() as isize,
    };
    let res;
    unsafe {res = handleInput(input1);};
    println!("{}", res);



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