use std::{fs::OpenOptions, io::Write, time::Instant};
#[cfg(feature = "cranelift")]
use wasmer::Cranelift;
#[cfg(feature = "singlepass")]
use wasmer::Singlepass;
#[cfg(feature = "llvm")]
use wasmer::LLVM;
use wasmer::{imports, Instance, Module, Store};
// use wasmer_compiler_cranelift::Cranelift;
// use wasmer_compiler_llvm::LLVM;

// use wasmer_compiler_singlepass::Singlepass;
#[cfg(feature = "singlepass")]
pub fn singlepass(wasm_bytes: &Vec<u8>, path: &str, iteration: &crate::Iterations) {
    //let path = "results-wasmer.txt";
    println!("Wasmer Singlepass");
    let compiler = Singlepass::default();
    let store = Store::new(compiler);
    run(store, wasm_bytes, path, iteration);
}
#[cfg(feature = "cranelift")]
pub fn cranelift(wasm_bytes: &Vec<u8>, path: &str, iteration: &crate::Iterations) {
    // Use Cranelift compiler with the default settings
    println!("Wasmer Cranelift");
    let compiler = Cranelift::default();

    // Create the store
    let store = Store::new(compiler);

    run(store, wasm_bytes, path, iteration);
}
#[cfg(feature = "llvm")]
pub fn llvm(wasm_bytes: &Vec<u8>, path: &str, iteration: &crate::Iterations) {
    // Use Cranelift compiler with the default settings
    println!("Wasmer LLVM");
    let compiler = LLVM::default();
    // Create the store
    let store = Store::new(compiler);

    run(store, wasm_bytes, path, iteration);
}

fn run(mut store: Store, wasm_bytes: &Vec<u8>, path: &str, iteration: &crate::Iterations) {
    _ = std::fs::remove_file(path);
    let mut output = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)
        .unwrap();

    let module = Module::new(&store, wasm_bytes).expect("Failed to create module");
    let import_object = imports! {};
    let instance_wasmer = Instance::new(&mut store, &module, &import_object)
        .expect("Failed to instantiate with Wasmer");

    let add_func_wasmer = instance_wasmer
        .exports
        .get_function("add")
        .expect("Failed to find function 'add'");
    let factorial_func_wasmer = instance_wasmer
        .exports
        .get_function("factorial")
        .expect("Failed to find function 'factorial'");
    let newton_sqrt_func_wasmer = instance_wasmer
        .exports
        .get_function("newton_sqrt")
        .expect("Failed to find function 'newton_sqrt'");
    let fibonacci_func_wasmer = instance_wasmer
        .exports
        .get_function("fibonacci")
        .expect("Failed to find function 'fibonacci'");

    //add
    let mut results = vec![];
    for i in 0..iteration.add {
        let start_time = Instant::now();
        let _result = add_func_wasmer
            .call(&mut store, &[i.into(), i.into()])
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //factorial
    let mut results = vec![];
    for i in 0..=iteration.factorial {
        let start_time = Instant::now();
        let _result = factorial_func_wasmer
            .call(&mut store, &[i.into()])
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //Newton's Method
    let mut results = vec![];
    for i in 0..iteration.newton {
        let start_time = Instant::now();
        let _result = newton_sqrt_func_wasmer
            .call(&mut store, &[(i as f64).into()])
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //fibonacci
    let mut results = vec![];
    for i in 0..iteration.fibonacci {
        let start_time = Instant::now();
        let _result = fibonacci_func_wasmer
            .call(&mut store, &[i.into()])
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();
}
