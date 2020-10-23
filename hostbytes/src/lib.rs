use std::mem;

extern "C" {
    fn get_host_bytes_size() -> u32;
    fn get_host_bytes(ptr: *const u8);
    fn get_host_bytes_with_buffer(index: u32, bufPtr: *const u8) -> u32;
    fn log_message(pointer: *const u8, length: u32);
}

const BUFFER_SIZE: usize = 1024;
static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];


// use me if engine doesnt support manually grow memory && copy data to memory
#[no_mangle]
pub extern fn allocate(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut u8
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut u8, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}


#[no_mangle]
pub extern "C" fn greet_with_size() {
    let size = unsafe { get_host_bytes_size() };
    let ptr = allocate(size as usize);
    unsafe { get_host_bytes(ptr) };
    let subject = unsafe { Vec::from_raw_parts(ptr, size as usize, size as usize) };
    let mut output = b"Hello, ".to_vec();
    output.extend(&subject);
    output.extend(&[b'!']);
    let hex_string = to_hex_string(output);

    unsafe{
        log_message(hex_string.as_ptr(), hex_string.len() as u32)
    }
}


#[no_mangle]
pub extern "C" fn greet_with_buffer() {
    let ptr = unsafe { BUFFER.as_ptr() };
    let mut vec: std::vec::Vec<u8> = Vec::with_capacity(0);
    let mut size;
    let mut index = 0;
    loop {
        size = unsafe { get_host_bytes_with_buffer(index, ptr) } as usize;
        if size == 0 {
            break
        }

        index += 1;
        vec.extend(unsafe { BUFFER[..size].iter() })
    }

    let mut output = b"Hello, ".to_vec();
    output.extend(vec);
    output.extend(&[b'!']);
    let hex_string = to_hex_string(output);

    unsafe{
        log_message(hex_string.as_ptr(), hex_string.len() as u32)
    }
}

pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
                                 .map(|b| format!("{:02X}", b))
                                 .collect();
    strs.join(" ")
}
  