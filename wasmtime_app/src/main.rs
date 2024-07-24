use wasmtime::*;

fn main() {
    for n in 1030..2000 {
        println!("Testing with {}kb of data", n);
        let mut store: Store<()> = Store::default();
        let module = Module::from_file(store.engine(), "./wasm_blob.wasm")
            .expect("Failed to read wasm file");
        let instance =
            Instance::new(&mut store, &module, &[]).expect("Failed to instantiate module");

        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("Failed to find memory");
        let entry_point = instance
            .get_typed_func::<(u32, u32), ()>(&mut store, "entry_point")
            .expect("Failed to find entry_point");

        let serialized_text =
            bincode::serialize(&"A".repeat(n * 1024)).expect("Failed to serialize");
        {
            let data_mut = memory.data_mut(&mut store);
            for (i, b) in serialized_text.iter().enumerate() {
                data_mut[i] = *b;
            }
        }
        entry_point
            .call(&mut store, (0, serialized_text.len() as u32))
            .expect("SHOULD WORK");
    }
}
