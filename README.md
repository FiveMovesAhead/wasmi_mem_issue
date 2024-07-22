# Memory Issue with WASMI

## Replicate Issue

Run command `bash run_test.sh`. Should result in error:
```
...
Testing with 1031kb of data
Testing with 1032kb of data
Testing with 1033kb of data
thread 'main' panicked at app/src/main.rs:46:13:
Error occured during execution: out of bounds memory access
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Description
1. Memory limits is set to 1GB
```
let limits = StoreLimitsBuilder::new()
            .memory_size(MAX_MEMORY)
            .memories(1)
            .trap_on_grow_failure(true)
            .build();
```

2. The wasm simply is deserializing some text data
```
#[no_mangle]
pub fn entry_point(ptr: *const u8) {
    let len: usize = unsafe { *(ptr as *const u32) } as usize;
    let data = unsafe { std::slice::from_raw_parts(ptr.add(4), len) };
    let text: String = bincode::deserialize(&data).expect("Failed to deserialize");
}
```

3. The app runs a loop incrementing `n` (the number of kb being passed to the wasm):
```
let serialized_text =
            bincode::serialize(&"A".repeat(n * 1024)).expect("Failed to serialize");
let memory = instance
    .get_memory(&store, "memory")
    .expect("Failed to find memory");
memory.grow(&mut store, NUM_PAGES.into()).unwrap();
memory
    .write(&mut store, 0, &(serialized_text.len() as u32).to_le_bytes())
    .expect("Failed to write to memory");
memory
    .write(&mut store, 4, &serialized_text)
    .expect("Failed to write to memory");
```

4. Despite memory limits set to 1GB, an `out of bounds memory access` occurs at 1033kb of data