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

fn invoke(vm: &mut Vm<Instance>, request: i32, request_size: i32) -> Vec<WasmValue> {
    let ret = vm.run_func(Some(""), "invoke", params!(request, request_size)).unwrap();
    return ret;
}

fn server_main() {
    // memoryを取得
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