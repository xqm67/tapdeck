fn main() {
    let _ = embed_resource::compile("res/tapdeck.rc", embed_resource::NONE);
    println!("cargo:rerun-if-changed=res/tapdeck.rc");
    println!("cargo:rerun-if-changed=res/icon.ico");
}
