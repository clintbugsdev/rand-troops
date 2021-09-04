extern crate libc;

use libc::size_t;
use rand::Rng;
use std::mem;

#[no_mangle]
pub extern "C" fn rand_generate(mut types: u32, mut size: u32, vec: *mut *mut u32) -> size_t {
    let mut generated: Vec<u32> = Vec::new();

    if types < size {
        let mut rng = rand::thread_rng();
        loop {
            if types > 1 {
                let rand_num = rng.gen_range(1..size - (types - 1));
                size -= rand_num;
                generated.push(rand_num);
            } else {
                generated.push(size);
            }
            types -= 1;
            // end of loop
            if types == 0 {
                break;
            }
        }
    }

    generated.shrink_to_fit();
    let ret = generated.len();
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

        Vec::from_raw_parts(arr, arr_size, arr_size)
    };
}
