#![allow(unused)] // silence unused warnings while exploring (to comment out)

extern crate itertools;
extern crate libc;
use libc::{c_char, size_t};
use rand::seq::SliceRandom;
use rand::Rng;
use redis::{Client, Commands, Connection, ConnectionLike, RedisResult};
use std::env;
use std::error::Error;
use std::ffi::CStr;
use std::mem;

use itertools::Itertools;

fn connect() -> RedisResult<Connection> {
    //format - host:port
    let redis_host_name = match env::var("ENVIRONMENT_VARIABLE") {
        Ok(v) => v.to_string(),
        Err(_) => "127.0.0.1".to_string(),
    };

    let redis_password = match env::var("REDIS_PASSWORD") {
        Ok(v) => v.to_string(),
        Err(_) => "".to_string(),
    };

    //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };

    let mut redis_conn_url = match redis_password.len() > 0 {
        true => format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name),
        false => format!("{}://{}", uri_scheme, redis_host_name),
    };

    let mut client = redis::Client::open(redis_conn_url);
    let mut conn = client.unwrap().get_connection();
    conn
}

fn rand_get_list(hashed_req_str: &str) -> Vec<String> {
    let mut conn = connect();
    let list: Vec<String> = match conn.is_ok() {
        true => conn.unwrap().smembers(hashed_req_str).unwrap(),
        _ => vec![],
    };
    println!("{:?}", list);
    list
}

fn rand_is_member(hashed_req_str: &str, item: &String) -> bool {
    let mut conn = connect();

    let is_member = match conn.is_ok() {
        true => redis::cmd("SISMEMBER")
            .arg(hashed_req_str)
            .arg(item)
            .query(&mut conn.unwrap())
            .unwrap(),
        _ => false,
    };

    is_member
}

fn rand_add(hashed_req_str: &str, item: &String) -> bool {
    let mut conn = connect();

    if conn.is_err() {
        return false;
    }

    let _: () = match conn.is_ok() {
        true => conn.unwrap().sadd(hashed_req_str, item).unwrap(),
        _ => (),
    };

    true
}

fn rand_del(hashed_req_str: &str) -> bool {
    let mut conn = connect();

    if conn.is_err() {
        return false;
    }

    let _: () = match conn.is_ok() {
        true => conn.unwrap().del(hashed_req_str).unwrap(),
        _ => (),
    };

    true
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
                    types = orig_types;
                    size = orig_size;
                    generated.clear();
                    // delete random list if length was equal to the size
                    if rand_list.len() as u32 >= orig_types {
                        rand_del(hashed_req_str);
                    }
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
