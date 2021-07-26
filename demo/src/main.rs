use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::os::raw::c_char;
use std::ffi::CStr;

#[derive(Debug)]
#[repr(C)]
pub struct GoString {
    pub p: *const c_char,
    pub n: isize,
}

#[link(name = "opa")]
extern {
    // fn DoubleInput(input: libc::c_int) -> libc::c_int;
    // fn Hello();
    // pub fn handleReference(mr_ref: GoString);
    // pub fn makeDecisionGo(policy: GoString, message: GoString) -> GoString;
    pub fn makeDecisionGo(policy: GoString, message: GoString) -> *mut c_char;

}

fn main() {

    // println!("Hello, world!");
    let s1 = "test.rego";
    let s2 = r#"
            {
                "mrEnclave" : "123",
                "mrSigner" : "456",
                "productId" : "789"
            }
            "#;
    let result = set_reference(s1, s2);


    let policy = "package demo\n\n".to_owned() + "\n
    default allow = false
    
    allow = true {
        1==1
    }
        ";
    let result = set_raw_policy("test1.rego", &policy);

    let result = export_policy("test1.rego");
    println!("{}", result);

    println!("================================================");

    let input = "{\"mrEnclave\":\"123\",\"mrSigner\":\"456\",\"productId\":\"789\"}";
    let result = make_decision("test.rego", input);
    println!("======{}", result);

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
}";

    let path = String::from("src/policy/") + policy_name;
    let path = Path::new(&path);
    let display = path.display();

    // Open the file in write-only mode, return `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => {
            println!("couldn't create {}: {}", display, why.description());
            return false;
        },
        Ok(file) => file,
    };

    // Write the string `policy` into `file`, return `io::Result<()>`
    match file.write_all(policy.as_bytes()) {
        Err(why) => {
            println!("couldn't write to {}: {}", display, why.description());
            return false;
        },
        Ok(_) => (),
    }

    true
}

pub fn set_raw_policy(policy_name: &str, policy: &str)-> bool {

    let path = String::from("src/policy/") + policy_name;
    let path = Path::new(&path);
    let display = path.display();

    // Open the file in write-only mode, return `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => {
            println!("couldn't create {}: {}", display, why.description());
            return false;
        },
        Ok(file) => file,
    };

    // Write the string `policy` into `file`, return `io::Result<()>`
    match file.write_all(policy.as_bytes()) {
        Err(why) => {
            println!("couldn't write to {}: {}", display, why.description());
            return false;
        },
        Ok(_) => (),
    }

    let status = match Command::new("opa").arg("check").arg(&path).status(){
        Err(why) => {
            println!("failed to check");
            return false
        }
        Ok(res) => res,
    };

    if !status.success(){
        println!("hhh");
        return false;
    }

    true
}

pub fn export_policy(policy_name: &str)-> String {
    let path = String::from("src/policy/") + policy_name;

    let mut contents = String::new();

    let mut file = match File::open(path) {
        Err(why) => {
            println!("failed to open");
            return contents;
        }
        Ok(res) => res,
    };

    match file.read_to_string(&mut contents) {
        Err(why) => {
            println!("failed to read");
            return contents;
        }
        Ok(res) => res,
    };

    println!("{}", contents);
    contents
}

pub fn make_decision(policy_name : &str, message : &str) -> String {

    let h1 = message.as_ptr() as *const i8;
    let input1 = GoString {
        p: h1,
        n: message.len() as isize,
    };
    let result = export_policy(policy_name);

    let h2 = result.as_ptr() as *const i8;
    let input2 = GoString {
        p: h2,
        n: result.len() as isize,
    };
    let res: GoString;
    // unsafe {res = makeDecisionGo(input2, input1);};

    // let v: &[u8] = unsafe { std::slice::from_raw_parts(res.p as *const u8, res.n as usize) };
    // let line = String::from_utf8(v.to_vec()).unwrap();


    let c_buf: *mut c_char = unsafe { makeDecisionGo(input2, input1) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();  // if necessary
    println!("==={}", &str_buf);

    str_buf
}