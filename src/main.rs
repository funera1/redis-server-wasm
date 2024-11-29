use std::collections::HashMap;

use wasmedge_sdk::{params, Instance, Module, Store, Vm, WasmVal, WasmValue, VmBuilder, config::ConfigBuilder};


fn allocate(vm: &mut Vm, size: i32, type_tag: i32) -> Vec<WasmValue> {
    // let params = params!(size, type_tag);
    let ret = vm.run_func(Some(""), "allocate", params!(size, type_tag)).expect("failed to allocate");
    return ret;
}

fn release_objects(vm: &mut Vm) {
    let _ = vm.run_func(Some(""), "release_objects", params!()).expect("failed to release_objects");
}

fn get_result_ptr(vm: &mut Vm) -> Vec<WasmValue> {
    let ret = vm.run_func(Some(""), "get_result_ptr", params!()).expect("failed to get_result_ptr");
    return ret;
}

fn get_result_size(vm: &mut Vm) -> Vec<WasmValue> {
    let ret = vm.run_func(Some(""), "get_result_size", params!()).expect("failed to get_result_size");
    return ret;
}

fn set_result_ptr(vm: &mut Vm, ptr: i32) {
    let _ = vm.run_func(Some(""), "set_result_ptr", params!(ptr)).expect("failed to set_result_ptr");
}

fn set_result_size(vm: &mut Vm, size: i32) {
    let _ = vm.run_func(Some(""), "set_result_size", params!(size)).expect("failed to set_result_size");
}

fn invoke(vm: &mut Vm, request: i32, request_size: i32) {
    let _ = vm.run_func(Some(""), "invoke", params!(request, request_size)).unwrap();
}

fn server_main(vm: &mut Vm) {
    // 1. memoryを取得
    let mut memory = vm.active_module().expect("Not found active_module")
                        .memory("").expect("Not found memory");
    // 2. memory領域を確保＆ポインタを取得
    let request ="PING";
    let req_ptr = allocate(vm, request.len() as i32, 0)[0];

    // 333. memoryにrequestを書き込む
    memory[req_ptr.to_i32()] = request;

    // 4. invokeにmemoryを渡す
    invoke(vm, req_ptr.to_i32(), request.len() as i32);

    // 5. get_result_ptrでinvokeの結果を取得
    let response = get_result_ptr(vm)[0];
}

fn main() {
    let wasm_lib_file = "wasm/redis.wasm";

    // create a config with the `wasi` option enabled
    let config = ConfigBuilder::default()
                            .build().expect("Failed to config build");

    // create a VM with the config
    let mut vm = VmBuilder::new().with_config(config)
                        .build().expect("Failed to vmbuild");

    let res = vm
        .register_module_from_file("wasm-lib", &wasm_lib_file)
        .expect("Failed to register module");

    server_main(&mut vm);
    //     .run_func(Some("wasm-lib"), "fib", params!(num))?;
    // println!("fib({}) = {}", num, res[0].to_i32());
}
