# Getting Started

This guide will help you get your system set up for building this project. 
### Tip
For best result and running output from this project. Please use Windows Subsystem for Linux when using Windows OS or run it in any the Linux or MacOS machine.

## Prerequisites

#### Node.js

This project requires Node.js and supports the latest Node version and all LTS releases. If you don't already have [Node.js](https://nodejs.org/) installed, or don't have a supported version, go to the for installation instructions.

#### Rust Programming Language

The project requires Rust for development. If you don't already have Rust installed, or don't have a supported version, go to the [Rust](https://www.rust-lang.org/install.html) web site for installation instructions.

Rust may have additional dependencies depending on your target platform. For example, Visual Studio on Windows.

#### Redis (Optional)

The project optionally requires Redis for temporary storage of generated random distributed troops. If you don't already have Redis installed, or don't have a supported version, go to the [Redis](https://redis.io/topics/quickstart) web site for installation instructions.


## Setup
#### 1. Run Redis-Server 
Open a new terminal window and let redis server run on your local machine while working on this project. 
###### Note: The project uses the default hostname and port.
```bash
redis-server
```
#### 2. Cargo
Use the package manager [cargo](https://doc.rust-lang.org/cargo/) to download the Rust package's dependencies and makes distributable package.
```bash
# Clean the installed rust packages if exists
cargo clean
```
```bash
# Build the rust library
cargo build --release
```
#### 3. NPM
This command installs a package and any packages that it depends on.
```bash
npm install
```

## Sample Usage
#### 
```javascript
// Filename index.js

// import random-troops.js module
const randomTroops = require("./random-troops");

// set the parameters
let player = "Player 1";
let armyTypes = ["Spearmen", "Swordsmen", "Archer"];
let armySize = 10;

// call the randomTroops fund and prints result
console.log(randomTroops(player, armyTypes, armySize));
```

## Actual Written Codes
##### Node.js
#### 
```javascript
// Filename random-troops.js

const crypto = require("crypto");
const ffi = require("ffi-napi");
const ref = require("ref-napi");

const arrayType = require("ref-array-di")(ref);

// define the "int[]" type
const randTroopsArray = arrayType("uint32");
const randTroopsArrayPtr = ref.refType(randTroopsArray);

// Rust library path
const randGenlibPath = "target/release/librand_gen.so";
const lib = ffi.Library(randGenlibPath, {
  rand_generate: ["size_t", ["string", "uint32", "uint32", randTroopsArrayPtr]],
  rand_free: ["void", [randTroopsArray, "size_t"]],
});

function randomTroops(player, armyTypes, armySize) {
  const reqStr = `${player}${armyTypes.join("")}${armySize}`;
  const sha256Hasher = crypto.createHmac("sha256", reqStr.replace(/\s/g, ""));
  const hashedReq = sha256Hasher.update(reqStr).digest("base64");
  const bufPtr = ref.alloc(randTroopsArrayPtr);
  const arraySize = lib.rand_generate(
    hashedReq,
    armyTypes.length,
    armySize,
    bufPtr
  );
  const armies = randTroopsArray(bufPtr.deref());
  armies.length = arraySize;

  try {
    let result = [];
    for (let i = 0; i < armies.length; i++) {
      result.push({ name: armyTypes[i], troops: armies[i] });
    }
    return result;
  } finally {
    lib.rand_free(armies, arraySize);
  }
}

module.exports = randomTroops;

```
##### Rust Programming Language
#### 
```rust
// src/lib.rs

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
    // println!("{:?}", list);
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

    // println!("{}", item);

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
                // shuffle result
                generated.shuffle(&mut rand::thread_rng());
                // convert numbers to strings
                let mut merged_generated = generated.iter().join("-");
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

```
## Test
#### 
```bash
# Run random-troops.test.js
npm jest
```
