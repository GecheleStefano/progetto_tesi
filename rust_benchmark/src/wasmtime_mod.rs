use std::{fs::OpenOptions, io::Write, time::Instant};

use wasmtime::{Engine, Instance, Module, Store};

pub fn wasmtime(wasm_bytes: &[u8], path: &str, iteration: &crate::Iterations) {
    println!("Wasmtime");
    _ = std::fs::remove_file(path);
    let mut output = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)
        .unwrap();

    let engine = Engine::default();
    let module = Module::from_binary(&engine, wasm_bytes).unwrap();
    let mut store = Store::new(&engine, 4);
    let instance_wasmtime =
        Instance::new(&mut store, &module, &[]).expect("Failed to instantiate with Wasmtime");

    let add_func_wasmtime = instance_wasmtime
        .get_typed_func::<(i32, i32), i32>(&mut store, "add")
        .expect("Failed to find function 'add'");
    let factorial_func_wasmtime = instance_wasmtime
        .get_typed_func::<i64, i64>(&mut store, "factorial")
        .expect("Failed to find function 'factorial'");
    let newton_func_wasmtime = instance_wasmtime
        .get_typed_func::<f64, f64>(&mut store, "newton_sqrt")
        .expect("Failed to find function 'newton_sqrt'");
    let fibonacci_func_wasmtime = instance_wasmtime
        .get_typed_func::<i32, i32>(&mut store, "fibonacci")
        .expect("Failed to find function 'fibonacci'");

    //add
    let mut results = vec![];
    for i in 0..iteration.add {
        let start_time = Instant::now();
        let _result = add_func_wasmtime
            .call(&mut store, (i, i))
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //factorial
    let mut results = vec![];
    for i in 0..iteration.factorial {
        let start_time = Instant::now();
        let _result = factorial_func_wasmtime
            .call(&mut store, i)
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //Newton's Method
    let mut results = vec![];
    for i in 0..iteration.newton {
        let start_time = Instant::now();
        let _result = newton_func_wasmtime
            .call(&mut store, i.into())
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //fibonacci
    let mut results = vec![];
    for i in 0..iteration.fibonacci {
        let start_time = Instant::now();
        let _result = fibonacci_func_wasmtime
            .call(&mut store, i)
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();
}

