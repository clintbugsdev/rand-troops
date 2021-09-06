#![allow(unused)] // silence unused warnings while exploring (to comment out)

extern crate itertools;
extern crate libc;
use libc::{c_char, size_t};
use rand::seq::SliceRandom;
use rand::Rng;
use redis::Commands;
use std::error::Error;
use std::ffi::CStr;
use std::mem;

use itertools::Itertools;

fn connect() -> redis::Connection {
    redis::Client::open("redis://127.0.0.1/")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

fn rand_get_list(hashed_req_str: &str) -> Vec<String> {
    let mut conn = connect();

    let list: Vec<String> = conn
        .smembers(hashed_req_str)
        .expect("failed to execute SMEMBERS");
    println!("{:?}", list);
    list
}

fn rand_is_member(hashed_req_str: &str, item: &String) -> bool {
    let mut conn = connect();

    let is_member: bool = redis::cmd("SISMEMBER")
        .arg(hashed_req_str)
        .arg(item)
        .query(&mut conn)
        .expect("failed to execute SISMEMBER");

    is_member
}

fn rand_add(hashed_req_str: &str, item: &String) -> Result<(), Box<dyn Error>> {
    let mut conn = connect();

    let _: () = conn
        .sadd(hashed_req_str, item)
        .expect("failed to execute SADD");

    Ok(())
}

fn rand_del(hashed_req_str: &str) -> Result<(), Box<dyn Error>> {
    let mut conn = connect();

    let _: () = conn.del(hashed_req_str).expect("failed to execute DEL");

    Ok(())
}

#[no_mangle]
pub extern "C" fn rand_generate(
    hashed_req: *const c_char,
    mut types: u32,
    mut size: u32,
    vec: *mut *mut u32,
) -> size_t {
    // get hashed request from C string pointer
    let c_str = unsafe {
        assert!(!hashed_req.is_null());
        CStr::from_ptr(hashed_req)
    };
    // set to string
    let hashed_req_str = c_str.to_str().unwrap();
    let rand_list = rand_get_list(hashed_req_str);

    // initiate variable for random numbers
    let orig_types = types.clone();
    let orig_size = size.clone();
    let mut generated: Vec<u32> = Vec::new();
    if types < size {
        let mut rng = rand::thread_rng();
        // loop to get random numbers
        loop {
            if types > 1 {
                let rand_num = rng.gen_range(1..=size - (types - 1));
                size -= rand_num;
                generated.push(rand_num);
            } else {
                generated.push(size);
            }
            types -= 1;
            // end of loop
            if types == 0 {
                // convert numbers to strings
                let mut merged_generated = generated.clone().iter().join("-");
                // is member then reset variables to go back to loop
                if rand_is_member(hashed_req_str, &merged_generated) {
                    // delete random list if length was equal to the size
                    if rand_list.len() as u32 >= orig_types {
                        rand_del(hashed_req_str);
                    }
                    types = orig_types;
                    size = orig_size;
                    println!("{:?}", generated);
                    generated.clear();
                } else {
                    // add generated random numbers to Redis
                    rand_add(hashed_req_str, &merged_generated);
                    break;
                }
            }
        }
    }
    let ret = generated.len();
    // set generated random numbers as pointer
    unsafe { *vec = generated.as_mut_ptr() };
    mem::forget(generated);

    ret
}

#[no_mangle]
pub extern "C" fn rand_free(arr: *mut u32, arr_size: size_t) {
    unsafe {
        if arr.is_null() {
            return;
        }
        // free the generated random numbers array pointer
        Vec::from_raw_parts(arr, arr_size, arr_size)
    };
}
