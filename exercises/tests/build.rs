// build.rs
fn main() {
    // tests7 的环境变量设置
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:rerun-if-changed=build.rs");

    // tests8 的特性启用 (新增部分)
    println!("cargo:rustc-cfg=feature=\"pass\"");
}