use wasmedge_sdk::{config::{ConfigBuilder, StatisticsConfigOptions}, params, NeverType, Vm, VmBuilder, WasmVal, WasmValue};
use clap::{Parser, ArgAction};
// use env_logger;
// use log::{debug, error};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("./"))]
    pub image_dir: String,
    // #[arg(default_value_t = false)]
    #[arg(short, long, action(ArgAction::SetTrue))]
    pub restore_flag: bool,
}


fn allocate(vm: &Vm<NeverType>, size: i32, type_tag: i32) -> Vec<WasmValue> {
    // let params = params!(size, type_tag);
    let ret = vm.run_func(Some("redis-core"), "allocate", params!(size, type_tag)).expect("failed to allocate");
    return ret;
}

fn release_objects(vm: &Vm<NeverType>) {
    let _ = vm.run_func(Some("redis-core"), "release_objects", params!()).expect("failed to release_objects");
}

fn get_result_ptr(vm: &Vm<NeverType>) -> Vec<WasmValue> {
    let ret = vm.run_func(Some("redis-core"), "get_result_ptr", params!()).expect("failed to get_result_ptr");
    return ret;
}

fn get_result_size(vm: &Vm<NeverType>) -> Vec<WasmValue> {
    let ret = vm.run_func(Some("redis-core"), "get_result_size", params!()).expect("failed to get_result_size");
    return ret;
}

// fn set_result_ptr(vm: &Vm, ptr: i32) {
//     let _ = vm.run_func(Some("redis-core"), "set_result_ptr", params!(ptr)).expect("failed to set_result_ptr");
// }

// fn set_result_size(vm: &Vm, size: i32) {
//     let _ = vm.run_func(Some("redis-core"), "set_result_size", params!(size)).expect("failed to set_result_size");
// }

fn invoke(vm: &Vm<NeverType>, request: i32, request_size: i32) {
    let _ = vm.run_func(Some("redis-core"), "invoke", params!(request, request_size)).unwrap();
}

pub fn query_and_response(vm: &Vm<NeverType>, request: &str) -> String {
    // 1. memoryを取得
    let mut memory = vm.named_module("redis-core")
                            .expect("Not found active_module")
                            .memory("memory").expect("Not found memory");

    // 2. memory領域を確保＆ポインタを取得
    // let request ="PING";
    let req_ptr = allocate(vm, request.len() as i32, 0)[0];

    // 3. memoryにrequestを書き込む
    let _ = memory.write(request, req_ptr.to_i32() as u32);

    // 4. invokeにmemoryを渡す
    invoke(vm, req_ptr.to_i32(), request.len() as i32);

    // 5. get_result_ptrでinvokeの結果を取得
    let response_ptr = get_result_ptr(vm)[0];
    let response_size = get_result_size(vm)[0];
    let resp_bytes =memory.read(response_ptr.to_i32() as u32, response_size.to_i32() as u32).expect("Failed to read memory");
    let response = String::from_utf8(resp_bytes).unwrap();
    // println!("{}", String::from_utf8(response).unwrap());

    // 6. memory開放
    let _ = release_objects(vm);

    return response;
}

pub fn init_redis_core(option: &Args) -> Vm<NeverType> {
    let wasm_lib_file = "wasm/redis.wasm";

    let options = StatisticsConfigOptions::new()
                                            .image_dir(option.image_dir.clone())
                                            .restore_flag(option.restore_flag);

    // create a config with the `wasi` option enabled
    let config = ConfigBuilder::default()
                            .with_statistics_config(options)
                            .build().expect("Failed to config build");

    // create a VM with the config
    let vm_base = VmBuilder::new().with_config(config)
                        .build().expect("Failed to vmbuild");

    // NOTE: active_instance以外マイグレーションできない可能性あり
    let vm = vm_base
        .register_module_from_file("redis-core", &wasm_lib_file)
        .expect("Failed to register module");

    return vm;
}
