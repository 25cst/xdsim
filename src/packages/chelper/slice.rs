use std::{
    ffi::{CStr, CString, c_void},
    ptr,
};

use xdsim_cbinds::common::{Slice, Str};

extern "C" fn vec_drop_rustonly<T>(vec_ptr: *mut c_void, len: u64) {
    let thin_ptr = vec_ptr as *mut T;
    let fat_ptr: *mut [T] = ptr::slice_from_raw_parts_mut(thin_ptr, len as usize);
    let _boxed_slice: Box<[T]> = unsafe { Box::from_raw(fat_ptr) };
    // dropped
}

/// Converts a vec into a Slice
/// the slice should only contain elements that frees up all memory when dropped
/// - including primitives and anything declared in rust
/// - but not pointers malloced in C
pub fn from_vec_rustonly<T>(vec: Vec<T>) -> Slice {
    Slice {
        length: vec.len() as u64,
        first: Box::into_raw(vec.into_boxed_slice()) as *mut c_void,
        drop: vec_drop_rustonly::<T>,
    }
}

pub fn from_str(original: &Str) -> String {
    unsafe { CString::from_raw(original.first) }
        .to_string_lossy()
        .to_string()
}

pub fn from_slice<T>(original: &Slice) -> &[T] {
    unsafe { std::slice::from_raw_parts(original.first as *const T, original.length as usize) }
}
