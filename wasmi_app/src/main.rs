use wasmi::{Config, Engine, Linker, Module, Store, StoreLimitsBuilder};

const MAX_MEMORY: usize = 1 * 1024 * 1024 * 1024; // 1GB

fn main() {
    println!("Loading ./wasm_blob.wasm");
    let wasm = std::fs::read("./wasm_blob.wasm").expect("Failed to read wasm file");
    for n in 1030..2000 {
        println!("Testing with {}kb of data", n);
        let config = Config::default();
        let limits = StoreLimitsBuilder::new()
            .memory_size(MAX_MEMORY)
            .memories(1)
            .trap_on_grow_failure(true)
            .build();
        let engine = Engine::new(&config);
        let mut store = Store::new(&engine, limits);
        store.limiter(|lim| lim);
        let linker = Linker::new(&engine);
        let module =
            Module::new(store.engine(), &*wasm).expect("Wasmi: failed to instantiate module");
        let instance = &linker
            .instantiate(&mut store, &module)
            .expect("Wasmi: failed to instantiate linker")
            .start(&mut store)
            .expect("Wasmi: failed to start module");
        let serialized_text =
            bincode::serialize(&"A".repeat(n * 1024)).expect("Wasmi: failed to serialize");
        let memory = instance
            .get_memory(&store, "memory")
            .expect("Wasmi: failed to find `memory`");
        let init = instance
            .get_typed_func::<u32, u32>(&store, "init")
            .expect("Wasmi: failed to find `init` function");
        let deinit = instance
            .get_typed_func::<(u32, u32), ()>(&store, "deinit")
            .expect("Wasmi: failed to find `deinit` function");
        let deserialize = instance
            .get_typed_func::<(u32, u32), ()>(&store, "deserialize")
            .expect("Wasmi: failed to find `deserialize` function");
        let data_len: u32 = serialized_text.len() as u32;
        let data_ptr: u32 = init.call(&mut store, data_len).unwrap();
        memory
            .write(&mut store, data_ptr as usize, &serialized_text)
            .expect("Wasmi: failed to write serialized text to `memory`");
        deserialize.call(&mut store, (data_ptr, data_len)).unwrap();
        deinit.call(&mut store, (data_ptr, data_len)).unwrap();
    }
}
