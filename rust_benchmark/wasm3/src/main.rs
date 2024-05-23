use std::{fs::OpenOptions, io::Write, time::Instant};
use wasm3::{Environment, Module};

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

    wasm3(&wasm_bytes, "results/wasm3.txt", &iteration);
}

pub fn wasm3(wasm_bytes: &[u8], path: &str, iteration: &crate::Iterations) {
    println!("Wasm3");
    _ = std::fs::remove_file(path);
    let mut output = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)
        .unwrap();

    let env = Environment::new().expect("Unable to create environment");
    let rt = env.create_runtime(1024).expect("Unable to create runtime");
    let module = Module::parse(&env, wasm_bytes).expect("Unable to parse module");

    let module = rt.load_module(module).expect("Unable to load module");

    let add_func_wasm3 = module
        .find_function::<(i32, i32), i32>("add")
        .expect("Unable to find function");
    let factorial_func_wasm3 = module
        .find_function::<i64, i64>("factorial")
        .expect("Unable to find function");
    let newton_func_wasm3 = module
        .find_function::<f64, f64>("newton_sqrt")
        .expect("Unable to find function");
    let fibonacci_func_wasm3 = module
        .find_function::<i32, i32>("fibonacci")
        .expect("Unable to find function");
    // println!("Wasm says that 3 + 6 is {}", func.call(3, 6).unwrap());

    //add
    let mut results = vec![];
    for i in 0..iteration.add {
        let start_time = Instant::now();
        let _result = add_func_wasm3.call(i, i).expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //factorial
    let mut results = vec![];
    for i in 0..iteration.factorial {
        let start_time = Instant::now();
        let _result = factorial_func_wasm3
            .call(i)
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //Newton's Method
    let mut results = vec![];
    for i in 0..iteration.newton {
        let start_time = Instant::now();
        let _result = newton_func_wasm3
            .call(i.into())
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //fibonacci
    let mut results = vec![];
    for i in 0..(iteration.fibonacci - 15) {
        let start_time = Instant::now();
        let _result = fibonacci_func_wasm3
            .call(i)
            .expect("Failed to call function");
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();
}
