use core::slice;

/// Allocates a new boxed slice on the Wasm side and returns a raw pointer to it.
#[no_mangle]
pub fn init(len: u32) -> *mut u8 {
    Box::leak(vec![0x00_u8; len as usize].into_boxed_slice()).as_mut_ptr()
}

/// Deallocates the boxed slice allocated via [`init`].
///
/// The `len` must match the `len` given to [`init`].
#[no_mangle]
pub fn deinit(data: *mut u8, len: u32) {
    let len = len as usize;
    let vec = unsafe { Vec::from_raw_parts(data, len, len) };
    drop(vec)
}

/// Deserializes the `String` from the slice allocated via [`init`].
///
/// The `len` must match the `len` given to [`init`].
#[no_mangle]
pub fn deserialize(ptr: *mut u8, len: u32) {
    let data = unsafe { slice::from_raw_parts(ptr, len as usize) };
    let _text: String = bincode::deserialize(data).expect("failed to deserialize");
}
