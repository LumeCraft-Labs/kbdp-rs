// build.rs — 嵌入应用程序图标
fn main() {
    let _ = embed_resource::compile("resources/app.rc", embed_resource::NONE);
}
