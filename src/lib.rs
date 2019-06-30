#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let vec: Vec<f32> = vec![0.0; size];
    Box::into_raw(vec.into_boxed_slice()) as *mut f32
}

mod ruffbox;

lazy_static! {
    static ref RUFF: Mutex<ruffbox::Ruffbox> = Mutex::new(ruffbox::Ruffbox::new());
}

#[no_mangle]
pub extern "C" fn process(out_ptr_l: *mut f32, out_ptr_r: *mut f32, size: usize) {
    let mut ruff = RUFF.lock().unwrap();

    let out_buf_l: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr_l, size)};
    let out_buf_r: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr_r, size)};

    // mono for now ... 
    let out = ruff.process();
    for i in 0..128 {
        out_buf_l[i] = out[i];
        out_buf_r[i] = out[i];
    }    
}

#[no_mangle]
pub extern "C" fn trigger(sample_number: usize) {
    let mut ruff = RUFF.lock().unwrap();
    //log!("{}",sample_number);
    ruff.trigger(sample_number);
}

#[no_mangle]
pub extern "C" fn load(sample_ptr: *mut f32, size: usize) -> usize {
    let mut ruff = RUFF.lock().unwrap();
    let in_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(sample_ptr, size)};
    ruff.load(in_buf)
}







