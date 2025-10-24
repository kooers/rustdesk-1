//定义传递Repository secrets到编译环境中方法
use std::env;
fn set_env(key: &str) {
    if let Some(val_os) = env::var_os(key) {
        if let Some(val) = val_os.to_str() {
            println!("cargo:rustc-env={}={}", key, val);
        }
    }
}
fn main() {
    //使用定义的方法获取Repository secrets到编译环境
    set_env("APP_NAME");
    set_env("RENDEZVOUS_SERVER");
    set_env("RELAY_SERVER");
    set_env("API_SERVER");
    set_env("RS_PUB_KEY");
    set_env("DEFAULT_PASSWORD");

    let out_dir = format!("{}/protos", env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
