use wasmi::{Config, Engine, Linker, Module, Store, StoreLimitsBuilder};

const MAX_MEMORY: usize = 1 * 1024 * 1024 * 1024; // 1GB
const NUM_PAGES: u16 = (MAX_MEMORY / (64 * 1024)) as u16;

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
        let module = Module::new(store.engine(), &*wasm).expect("Failed to instantiate module");
        let instance = &linker
            .instantiate(&mut store, &module)
            .expect("Failed to instantiate linker")
            .start(&mut store)
            .expect("Failed to start module");
        let serialized_text =
            bincode::serialize(&"A".repeat(n * 1024)).expect("Failed to serialize");
        let memory = instance
            .get_memory(&store, "memory")
            .expect("Failed to find memory");
        memory.grow(&mut store, (NUM_PAGES - 100).into()).unwrap();
        memory
            .write(&mut store, 0, &serialized_text)
            .expect("Failed to write to memory");
        let func = instance
            .get_func(&store, "entry_point")
            .expect("Failed to find entry_point");
        func.typed::<(u32, u32), ()>(&store)
            .expect("Failed to instantiate function")
            .call(&mut store, (0, serialized_text.len() as u32))
            .expect("SHOULD WORK");
    }
}
