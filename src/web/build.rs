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
            "-o",
            "./generated-api",
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
            "src/moudle_bindings",
            "--module-path",
            "spacetimedb/",
        ])
        .status()
        .expect("spacetimedb not found. Is spacetimedb installed?");

    if !status.success() {
        panic!("Generation failed");
    }
}
