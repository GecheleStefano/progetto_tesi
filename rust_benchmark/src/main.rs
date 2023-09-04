#[cfg(feature = "wasm3")]
mod wasm3_mod;
#[cfg(any(feature = "wasmedge", feature = "wasmedge_aot"))]
mod wasmedge_mod;
#[cfg(any(feature = "singlepass", feature = "cranelift", feature = "llvm"))]
mod wasmer_mod;
#[cfg(feature = "wasmtime")]
mod wasmtime_mod;
pub struct Iterations {
    add: i32,
    factorial: i64,
    newton: i32,
    fibonacci: i32,
}
fn main() {
    // Load WebAssembly module from file
    let wasm_bytes = std::fs::read("../wasm/target/wasm32-wasi/release/wasm.wasm")
        .expect("Failed to read WebAssembly file");
    // create direcotry results if not exist
    _ = std::fs::create_dir_all("results");

    let iteration = Iterations {
        add: 1_000_000,
        factorial: 20, //max 64 bit factorial
        newton: 1_000_000,
        fibonacci: 40,
    };

    // // Benchmark Wasmer
    #[cfg(feature = "singlepass")]
    wasmer_mod::singlepass(&wasm_bytes, "results/wasmer_singlepass.txt", &iteration);
    #[cfg(feature = "cranelift")]
    wasmer_mod::cranelift(&wasm_bytes, "results/wasmer_cranelift.txt", &iteration);
    #[cfg(feature = "llvm")]
    wasmer_mod::llvm(&wasm_bytes, "results/wasmer_llvm.txt", &iteration);

    // Benchmark Wasmtime
    #[cfg(feature = "wasmtime")]
    wasmtime_mod::wasmtime(&wasm_bytes, "results/wasmtime.txt", &iteration);

    // // Benchmark Wasmedge
    #[cfg(feature = "wasmedge")]
    wasmedge_mod::wasmedge(&wasm_bytes, "results/wasmedge.txt", &iteration);
    #[cfg(feature = "wasmedge_aot")]
    wasmedge_mod::wasmedge_aot(&wasm_bytes, "results/wasmedge_aot.txt", &iteration);

    // Benchmark Wasmtime
    #[cfg(feature = "wasm3")]
    wasm3_mod::wasm3(&wasm_bytes, "results/wasm3.txt", &iteration);
}
