use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("generate rust openapi-models");

    let status = Command::new("npx")
        .args([
            "@openapitools/openapi-generator-cli",
            "generate",
            "-i",
            "vs.swagger.yaml",
            "-g",
            "rust",
            // "-o",
            // "/models",
            "--global-property",
            "models",
            "--skip-validate-spec",
        ])
        .status()
        .expect("Npx not found. Is noded.js installed?");

    if !status.success() {
        panic!("Generation failed");
    }
    println!("generate markdown openapi-documentation ");

    let models_dir = Path::new("src/models");
    let mod_rs_path = models_dir.join("mod.rs");

    let mut mod_rs_content = String::from("// Generated in build.rs\n");

    if let Ok(entries) = fs::read_dir(models_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if file_stem != "mod" {
                        mod_rs_content.push_str(&format!("pub mod {};\n", file_stem));
                        mod_rs_content.push_str(&format!("pub use {}::*;\n\n", file_stem));
                    }
                }
            }
        }
    }
    fs::write(mod_rs_path, mod_rs_content).expect("Can't write mod.rs");


    let status = Command::new("npx")
        .args([
            "@openapitools/openapi-generator-cli",
            "generate",
            "-i",
            "vs.swagger.yaml",
            "-g",
            "markdown",
            "-o",
            "../../doc/api",
            "--skip-validate-spec",
        ])
        .status()
        .expect("Npx not found. Is noded.js installed?");

    if !status.success() {
        panic!("Generation failed");
    }

    println!("generate rust spacetimedb client");

    let status = Command::new("spacetime")
        .args([
            "generate",
            "--lang",
            "rust",
            "--out-dir",
            "src/module_bindings",
            "--module-path",
            "spacetimedb/",
        ])
        .status()
        .expect("spacetimedb not found. Is spacetimedb installed?");

    if !status.success() {
        panic!("Generation failed");
    }
}
