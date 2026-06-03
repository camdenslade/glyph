use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;


#[derive(Parser)]
#[command(name = "glyph", about = "Build and package Glyph desktop apps", version)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create a new Glyph app project
    New {
        /// Project name
        name: String,
        /// Use the counter template (default)
        #[arg(long)]
        template: Option<String>,
    },

    /// Build the app (debug by default)
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Target triple (e.g. x86_64-pc-windows-msvc)
        #[arg(long)]
        target: Option<String>,
        /// Bundle into .app (macOS) or folder with .exe + manifest (Windows)
        #[arg(long)]
        bundle: bool,
    },

    /// Build and run the app
    Run {
        /// Run in release mode
        #[arg(long)]
        release: bool,
    },

    /// Show info about the current project
    Info,
}


fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("\x1b[31merror:\x1b[0m {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Cmd::New { name, template } => cmd_new(&name, template.as_deref()),
        Cmd::Build { release, target, bundle } => cmd_build(release, target.as_deref(), bundle),
        Cmd::Run { release } => cmd_run(release),
        Cmd::Info => cmd_info(),
    }
}


fn cmd_new(name: &str, _template: Option<&str>) -> Result<()> {
    let dir = PathBuf::from(name);
    if dir.exists() {
        bail!("directory '{}' already exists", name);
    }
    std::fs::create_dir_all(dir.join("src"))?;
    std::fs::create_dir_all(dir.join("assets"))?;

    // Cargo.toml
    std::fs::write(
        dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{name}"
path = "src/main.rs"

[dependencies]
glyph = {{ git = "https://github.com/camdenslade/glyph" }}
"#
        ),
    )?;

    // src/main.rs — minimal counter app
    std::fs::write(
        dir.join("src/main.rs"),
        format!(
            r#"use glyph::prelude::*;

fn main() {{
    let count = Signal::new(0i32);
    let scroll_y   = Signal::new(0.0f32);
    let max_scroll = Signal::new((-1.0f32, -1.0f32));

    App::run(
        move |_, _| {{
            let theme = dark_theme();
            let c1 = count.clone();
            let c2 = count.clone();
            let view = scroll(
                column(vec![
                    gap(SPACE_16),
                    text("{name}", TEXT_3XL)
                        .color(theme.text).into(),
                    gap(SPACE_4),
                    row(vec![
                        btn_secondary(&theme, "−", move || c1.set(c1.get() - 1)),
                        text(count.get().to_string(), TEXT_3XL)
                            .color(theme.text).into(),
                        btn(&theme, "+", move || c2.set(c2.get() + 1)),
                    ])
                    .gap(SPACE_4).align_center().into(),
                ])
                .gap(0.0).align_center().fill_width().into(),
                Signal::new(0.0),
                scroll_y.clone(),
                max_scroll.clone(),
            ).grow().into();
            (theme, view)
        }},
        dark_theme(),
        "{name}",
        800.0,
        600.0,
    );
}}
"#
        ),
    )?;

    // build.rs — embeds the DPI-aware manifest on Windows at compile time
    std::fs::write(dir.join("build.rs"),
        "fn main() {\n\
         #[cfg(target_os = \"windows\")]\n\
         {\n\
             let manifest = \"<?xml version=\\\"1.0\\\" encoding=\\\"UTF-8\\\" standalone=\\\"yes\\\"?>\\n\
<assembly xmlns=\\\"urn:schemas-microsoft-com:asm.v1\\\" manifestVersion=\\\"1.0\\\">\\n\
  <application xmlns=\\\"urn:schemas-microsoft-com:asm.v3\\\">\\n\
    <windowsSettings>\\n\
      <dpiAwareness xmlns=\\\"http://schemas.microsoft.com/SMI/2016/WindowsSettings\\\">PerMonitorV2</dpiAwareness>\\n\
      <activeCodePage xmlns=\\\"http://schemas.microsoft.com/SMI/2019/WindowsSettings\\\">UTF-8</activeCodePage>\\n\
    </windowsSettings>\\n\
  </application>\\n\
</assembly>\\n\";\n\
             let out = std::env::var(\"OUT_DIR\").unwrap();\n\
             let path = std::path::Path::new(&out).join(\"app.manifest\");\n\
             std::fs::write(&path, manifest).unwrap();\n\
             println!(\"cargo:rustc-link-arg-bins=/MANIFEST:EMBED\");\n\
             println!(\"cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}\", path.display());\n\
         }\n\
         println!(\"cargo:rerun-if-changed=build.rs\");\n\
         }\n"
    )?;

    // .gitignore
    std::fs::write(dir.join(".gitignore"), "/target\n")?;

    // Glyph.toml — app metadata used by `glyph build --bundle`
    std::fs::write(
        dir.join("Glyph.toml"),
        format!(
            r#"[app]
name = "{name}"
version = "0.1.0"
identifier = "com.example.{name}"
icon = "assets/icon.png"
"#
        ),
    )?;

    println!("\x1b[32m✓\x1b[0m Created '{name}'");
    println!("  cd {name}");
    println!("  glyph run");
    Ok(())
}


fn cmd_build(release: bool, target: Option<&str>, bundle: bool) -> Result<()> {
    let meta = read_glyph_toml().ok();
    let bin_name = meta.as_ref()
        .map(|m| m.app.name.clone())
        .unwrap_or_else(cargo_bin_name);

    // cargo build
    let mut args = vec!["build"];
    if release { args.push("--release"); }
    if let Some(t) = target { args.extend(["--target", t]); }
    cargo(&args)?;

    if bundle {
        let profile = if release { "release" } else { "debug" };
        let target_dir = target.map(|t| format!("target/{t}/{profile}"))
            .unwrap_or_else(|| format!("target/{profile}"));
        let exe = PathBuf::from(&target_dir).join(&bin_name);

        if cfg!(target_os = "macos") {
            bundle_macos(&exe, &bin_name, meta.as_ref())?;
        } else if cfg!(target_os = "windows") {
            bundle_windows(&exe, &bin_name)?;
        } else {
            println!("  bundling not supported on this platform yet");
        }
    } else {
        println!("\x1b[32m✓\x1b[0m Build complete");
    }

    Ok(())
}


fn cmd_run(release: bool) -> Result<()> {
    let mut args = vec!["run"];
    if release { args.push("--release"); }
    cargo(&args)
}


fn cmd_info() -> Result<()> {
    println!("\x1b[1mGlyph\x1b[0m — GPU-native desktop app framework for Rust");
    println!();
    if let Ok(meta) = read_glyph_toml() {
        println!("  App:        {}", meta.app.name);
        println!("  Version:    {}", meta.app.version);
        println!("  Identifier: {}", meta.app.identifier);
    } else {
        println!("  No Glyph.toml found — run from a Glyph project directory");
    }
    println!();
    // Installed toolchain
    let _ = cargo(&["--version"]);
    Ok(())
}


fn bundle_macos(exe: &Path, bin_name: &str, meta: Option<&GlyphMeta>) -> Result<()> {
    let app_name = meta.map(|m| m.app.name.as_str()).unwrap_or(bin_name);
    let bundle_id = meta.map(|m| m.app.identifier.as_str()).unwrap_or("com.example.app");
    let version   = meta.map(|m| m.app.version.as_str()).unwrap_or("0.1.0");

    let app_dir = PathBuf::from(format!("{app_name}.app/Contents/MacOS"));
    let res_dir = PathBuf::from(format!("{app_name}.app/Contents/Resources"));
    std::fs::create_dir_all(&app_dir)?;
    std::fs::create_dir_all(&res_dir)?;

    // Copy binary
    let dest = app_dir.join(bin_name);
    std::fs::copy(exe, &dest).context("copy binary")?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
    }

    // Copy icon if it exists
    let icon_src = PathBuf::from("assets/icon.png");
    if icon_src.exists() {
        std::fs::copy(&icon_src, res_dir.join("icon.png")).ok();
    }

    // Info.plist
    std::fs::write(
        PathBuf::from(format!("{app_name}.app/Contents/Info.plist")),
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>            <string>{app_name}</string>
  <key>CFBundleDisplayName</key>     <string>{app_name}</string>
  <key>CFBundleIdentifier</key>      <string>{bundle_id}</string>
  <key>CFBundleVersion</key>         <string>{version}</string>
  <key>CFBundleShortVersionString</key><string>{version}</string>
  <key>CFBundleExecutable</key>      <string>{bin_name}</string>
  <key>CFBundlePackageType</key>     <string>APPL</string>
  <key>NSHighResolutionCapable</key> <true/>
  <key>NSSupportsAutomaticGraphicsSwitching</key><true/>
  <key>LSMinimumSystemVersion</key>  <string>12.0</string>
</dict>
</plist>
"#
        ),
    )?;

    println!("\x1b[32m✓\x1b[0m Bundled → {app_name}.app");
    Ok(())
}


fn bundle_windows(exe: &Path, bin_name: &str) -> Result<()> {
    let dist = PathBuf::from("dist");
    std::fs::create_dir_all(&dist)?;
    let dest = dist.join(format!("{bin_name}.exe"));
    std::fs::copy(exe, &dest).context("copy exe")?;

    // Copy assets if they exist
    let assets = PathBuf::from("assets");
    if assets.exists() {
        copy_dir_all(&assets, &dist.join("assets"))?;
    }

    println!("\x1b[32m✓\x1b[0m Bundled → dist/{bin_name}.exe");
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}


#[derive(serde::Deserialize)]
struct GlyphMeta {
    app: AppMeta,
}

#[derive(serde::Deserialize)]
struct AppMeta {
    name: String,
    version: String,
    identifier: String,
    #[allow(dead_code)]
    icon: Option<String>,
}

fn read_glyph_toml() -> Result<GlyphMeta> {
    let raw = std::fs::read_to_string("Glyph.toml").context("Glyph.toml not found")?;
    toml::from_str(&raw).context("invalid Glyph.toml")
}


fn cargo(args: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .context("cargo not found")?;
    if !status.success() {
        bail!("cargo exited with {status}");
    }
    Ok(())
}

fn cargo_bin_name() -> String {
    // Read Cargo.toml in cwd and find the first [[bin]] name
    let raw = std::fs::read_to_string("Cargo.toml").unwrap_or_default();
    if let Ok(val) = raw.parse::<toml::Value>() {
        if let Some(bins) = val.get("bin").and_then(|b| b.as_array()) {
            if let Some(first) = bins.first() {
                if let Some(name) = first.get("name").and_then(|n| n.as_str()) {
                    return name.to_string();
                }
            }
        }
        // Fall back to [package] name
        if let Some(name) = val.get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
        {
            return name.to_string();
        }
    }
    "app".to_string()
}
