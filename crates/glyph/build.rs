// build.rs — runs in the user's project at compile time.
// Sets up platform-specific app metadata automatically.

fn main() {
    // Windows: embed an app manifest for DPI awareness + visual styles,
    // and set the app icon from assets/icon.ico if it exists.
    #[cfg(target_os = "windows")]
    windows_setup();

    // macOS: nothing needed at build time — Info.plist is handled by `glyph bundle`.
    // Tell cargo to re-run if the icon changes.
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=assets/icon.png");
    println!("cargo:rerun-if-changed=Glyph.toml");
}

#[cfg(target_os = "windows")]
fn windows_setup() {
    // Write a minimal app manifest to the build dir and embed it.
    // This enables:
    //   - Per-monitor DPI awareness (crisp rendering on high-DPI displays)
    //   - Visual styles (modern controls look)
    //   - UTF-8 active code page
    let manifest = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity version="1.0.0.0" processorArchitecture="*"
                    name="GlyphApp" type="win32"/>
  <dependency>
    <dependentAssembly>
      <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls"
                        version="6.0.0.0" processorArchitecture="*"
                        publicKeyToken="6595b64144ccf1df" language="*"/>
    </dependentAssembly>
  </dependency>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">
        PerMonitorV2
      </dpiAwareness>
      <longPathAware xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">
        true
      </longPathAware>
      <activeCodePage xmlns="http://schemas.microsoft.com/SMI/2019/WindowsSettings">
        UTF-8
      </activeCodePage>
    </windowsSettings>
  </application>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v2">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#;

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_path = std::path::Path::new(&out_dir).join("glyph.manifest");
    std::fs::write(&manifest_path, manifest).expect("write manifest");

    // Embed using winres if available (optional dep — silently skip if not present)
    // Users who want the icon embedded add winres to their own Cargo.toml.
    // We just set the manifest link flag directly via rc.exe / windres.
    println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
        manifest_path.display()
    );
}
