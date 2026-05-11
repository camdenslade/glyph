fn main() {
    // Load .env file at build time and re-export as compile-time env vars.
    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    if let Ok(contents) = std::fs::read_to_string(&env_path) {
        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some((k, v)) = line.split_once('=') {
                println!("cargo:rustc-env={}={}", k.trim(), v.trim());
            }
        }
    }
    println!("cargo:rerun-if-changed=.env");
}
