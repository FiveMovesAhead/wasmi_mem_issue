#[no_mangle]
pub fn entry_point(ptr: *mut u8, len: u32) {
    let data = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    let _text: String = bincode::deserialize(data).expect("failed to deserialize");
}
