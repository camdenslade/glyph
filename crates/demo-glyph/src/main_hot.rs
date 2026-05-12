use core_glyph::Theme;
use platform_glyph::HotApp;

fn main() {
    // The dylib is built as `libglyph_demo.dylib` (macOS) / `libglyph_demo.so` (Linux).
    // We locate it relative to the workspace root so the path works whether you
    // run from the workspace root or from inside the crate directory.
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();

    let lib_name = if cfg!(target_os = "macos") {
        "libglyph_demo.dylib"
    } else if cfg!(target_os = "windows") {
        "glyph_demo.dll"
    } else {
        "libglyph_demo.so"
    };

    let lib_path = workspace_root.join("target").join("debug").join(lib_name);
    let src_dir  = manifest_dir.join("src");

    println!("[glyph-demo-hot] watching  {}", src_dir.display());
    println!("[glyph-demo-hot] dylib at  {}", lib_path.display());
    println!("[glyph-demo-hot] edit any .rs file in src/ to trigger a hot-reload");

    HotApp::run(
        &src_dir,
        &lib_path,
        "glyph-demo",
        Theme::light(),
        "Glyph — Hot Reload",
        800.0,
        600.0,
    );
}
