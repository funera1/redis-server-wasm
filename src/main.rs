use wasmedge_sdk::{VmBuilder, config::ConfigBuilder};
mod lib;

fn main() {
    let wasm_lib_file = "wasm/redis.wasm";

    // create a config with the `wasi` option enabled
    let config = ConfigBuilder::default()
                            .build().expect("Failed to config build");

    // create a VM with the config
    let vm_base = VmBuilder::new().with_config(config)
                        .build().expect("Failed to vmbuild");

    // NOTE: active_instance以外マイグレーションできない可能性あり
    let vm = vm_base
        .register_module_from_file("redis-core", &wasm_lib_file)
        .expect("Failed to register module");

    lib::server_main(&vm);
    //     .run_func(Some("wasm-lib"), "fib", params!(num))?;
    // println!("fib({}) = {}", num, res[0].to_i32());
}
