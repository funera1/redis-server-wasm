use std::collections::HashMap;

use wasmedge_sdk::{params, wasi::WasiModule, Module, Store, Vm, WasmVal, Instance, WasmValue};


fn allocate(vm: &mut Vm<Instance>, size: i32, type_tag: i32) -> Vec<WasmValue> {
    // let params = params!(size, type_tag);
    let ret = vm.run_func(Some(""), "allocate", params!(size, type_tag)).expect("failed to allocate");
    return ret;
}

fn release_objects(vm: &mut Vm<Instance>) {
    let _ = vm.run_func(Some(""), "release_objects", params!()).expect("failed to release_objects");
}

fn get_result_ptr(vm: &mut Vm<Instance>) -> Vec<WasmValue> {
    let ret = vm.run_func(Some(""), "get_result_ptr", params!()).expect("failed to get_result_ptr");
    return ret;
}

fn get_result_size(vm: &mut Vm<Instance>) -> Vec<WasmValue> {
    let ret = vm.run_func(Some(""), "get_result_size", params!()).expect("failed to get_result_size");
    return ret;
}

fn set_result_ptr(vm: &mut Vm<Instance>, ptr: i32) {
    let _ = vm.run_func(Some(""), "set_result_ptr", params!(ptr)).expect("failed to set_result_ptr");
}

fn set_result_size(vm: &mut Vm<Instance>, size: i32) {
    let _ = vm.run_func(Some(""), "set_result_size", params!(size)).expect("failed to set_result_size");
}

fn invoke(vm: &mut Vm<Instance>, request: i32, request_size: i32) {
    let _ = vm.run_func(Some(""), "invoke", params!(request, request_size)).unwrap();
}

fn server_main(vm: &mut Vm<Instance>) {
    // 1. memory領域を確保＆ポインタを取得
    let mut memory = allocate(vm, 10, 0)[0];

    // 2. memoryにrequestを書き込む
    let request ="PING";
    memory = request as WasmValue;

    // 3. invokeにmemoryを渡す
    invoke(vm, memory.to_i32(), request.len() as i32);

    // 4. get_result_ptrでinvokeの結果を取得
    let response =get_result_ptr(vm)[0];
}

fn main() {
    let wasm_lib_file = "wasm/redis.wasm";
    let num: i32 = 2;

    let mut wasi_module = WasiModule::create(None, None, Some(vec![".:."])).unwrap();

    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());

    let store = Store::new(None, instances).unwrap();

    let mut vm = Vm::new(store);

    let module = Module::from_file(None, wasm_lib_file).unwrap();

    vm.register_module(Some(""), module).unwrap();

    let res = vm.run_func(Some(""), "invoke", params!(1, 1)).unwrap();
    println!("fib({}) = {}", num, res[0].to_i32());
}