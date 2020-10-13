use std::ffi::{CStr};
use std::mem;
use std::os::raw::{c_char, c_void};

extern "C" {
    fn host_string()->*mut c_char;
    fn log_message(pointer: *const u8, length: u32);
}

// use me if engine doesnt support manually grow memory && copy data to memory
#[no_mangle]
pub extern fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}


#[no_mangle]
pub extern "C" fn greet() {
    let ptr = unsafe {host_string()};
    let subject = unsafe { CStr::from_ptr(ptr).to_bytes().to_vec() };
    let mut output = b"Hello, ".to_vec();
    output.extend(&subject);
    output.extend(&[b'!']);

    unsafe{
        log_message(output.as_ptr(), output.len() as u32)
        
    }
}
