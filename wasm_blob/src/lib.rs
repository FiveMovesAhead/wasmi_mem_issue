#[no_mangle]
pub fn entry_point(ptr: *const u8) {
    let len: usize = unsafe { *(ptr as *const u32) } as usize;
    let data = unsafe { std::slice::from_raw_parts(ptr.add(4), len) };
    let text: String = bincode::deserialize(&data).expect("Failed to deserialize");
}
