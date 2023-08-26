use std::{fs::OpenOptions, io::Write, time::Instant};
#[cfg(feature = "wasmedge_aot")]
use wasmedge_sdk::{config::CompilerConfigOptions, Compiler, CompilerOutputFormat};
use wasmedge_sdk::{
    config::{self, CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, VmBuilder, WasmVal,
};

#[cfg(feature = "wasmedge")]
pub fn wasmedge(wasm_bytes: &Vec<u8>, path: &str, iteration: &crate::Iterations) {
    //let path = "results-wasmedge.txt";
    //File::create(path).expect("Failed to create path");
    // create a config with the `wasi` option enabled
    println!("Wasmedge");
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()
        .unwrap();

    run(wasm_bytes, config, path, iteration);
}

// aot mode
#[cfg(feature = "wasmedge_aot")]
pub fn wasmedge_aot(wasm_bytes: &Vec<u8>, path: &str, iteration: &crate::Iterations) {
    //let path = "results-wasmedge_aot.txt";
    //File::create(path).expect("Failed to create path");
    println!("Wasmedge_aot");
    let out_dir = std::env::current_dir().unwrap();
    // create a config with the `wasi` option enabled
    let aot_filename = "wasm_aot";

    // create a Config context
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_compiler_config(
            CompilerConfigOptions::default()
                .out_format(CompilerOutputFormat::Wasm)
                .optimization_level(wasmedge_types::CompilerOptimizationLevel::O3),
        )
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()
        .unwrap();

    // compile wasm to so for runing in the `aot` mode
    let compiler = Compiler::new(Some(&config)).unwrap();
    let aot_file_path = compiler
        .compile_from_bytes(wasm_bytes, aot_filename, out_dir)
        .unwrap();
    assert!(&aot_file_path.exists());
    #[cfg(target_os = "macos")]
    assert!(aot_file_path.ends_with("example_aot_fibonacci.dylib"));
    #[cfg(target_os = "linux")]
    assert!(aot_file_path.ends_with("wasm_aot.so"));
    #[cfg(target_os = "windows")]
    assert!(aot_file_path.ends_with("example_aot_fibonacci.dll"));

    let wasm_bytes = std::fs::read(&aot_file_path).expect("Failed to read WebAssembly file");

    //let vm = vm.register_module_from_bytes("extern", wasm_bytes).unwrap();

    run(&wasm_bytes, config, path, iteration);
    // // remove the generated aot file
    assert!(std::fs::remove_file(aot_file_path).is_ok());
}
//    assert_eq!(returns.len(), 1);

fn run(wasm_bytes: &Vec<u8>, config: config::Config, path: &str, iteration: &crate::Iterations) {
    _ = std::fs::remove_file(path);
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(path)
        .unwrap();

    let vm = VmBuilder::new()
        .with_config(config)
        .build()
        .unwrap()
        .register_module_from_bytes("extern", wasm_bytes)
        .unwrap();
    // add
    let mut results = vec![];
    for i in 0..iteration.add {
        let start_time = Instant::now();
        let _result = vm.run_func(Some("extern"), "add", params!(i, i)).unwrap();
        let time = start_time.elapsed();

        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //factorial
    let mut results = vec![];
    for i in 0..=iteration.factorial {
        let start_time = Instant::now();
        let _result = vm
            .run_func(Some("extern"), "factorial", params!(i))
            .unwrap();
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //Newton's Method
    let mut results = vec![];
    for i in 0..iteration.newton {
        let start_time = Instant::now();
        let _result = vm
            .run_func(Some("extern"), "newton_sqrt", params!(i as f64))
            .unwrap();
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();

    //Fibonacci
    let mut results = vec![];
    for i in 0..iteration.fibonacci {
        let start_time = Instant::now();
        let _result = vm
            .run_func(Some("extern"), "fibonacci", params!(i))
            .unwrap();
        let time = start_time.elapsed();
        results.push(time.as_nanos());
    }
    writeln!(output, "{:?}", results).unwrap();
}
