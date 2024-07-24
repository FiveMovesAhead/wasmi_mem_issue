# Memory Issue with WASM

## Replicate Issue

Run command `bash run_test.sh`. Should result in error:
```
Running wasmi_app
...
Testing with 1032kb of data
Testing with 1033kb of data
thread 'main' panicked at wasmi_app/src/main.rs:45:14:
SHOULD WORK: Error { kind: TrapCode(MemoryOutOfBounds) }
...


Running wasmtime_app
...
Testing with 1032kb of data
Testing with 1033kb of data
SHOULD WORK: error while executing at wasm backtrace:
    0: 0x6852 - <unknown>!<wasm function 86>
    1: 0x94b0 - <unknown>!<wasm function 96>

Caused by:
    0: memory fault at wasm address 0x41414145 in linear memory of size 0x110000
    1: wasm trap: out of bounds memory access
```

## Description
1. The wasm simply is deserializing some text data
```
#[no_mangle]
pub fn entry_point(ptr: u32) {
    let len: usize = unsafe { *(ptr as *const u32) } as usize;
    let ptr = ptr as *const u8;
    let data = unsafe { std::slice::from_raw_parts(ptr.add(4), len) };
    let text: String = bincode::deserialize(&data).expect("Failed to deserialize");
}
```

2. The app runs a loop incrementing `n` (the number of kb being passed to the wasm)