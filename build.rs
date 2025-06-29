use std::process::Command;

fn main() {
    // Only run apt-get commands when building on Render (they set this env var)
    if std::env::var("RENDER").is_ok() {
        // Run system commands to install dependencies
        let commands = [
            "apt-get update -y",
            "apt-get install -y build-essential pkg-config",
            "apt-get install -y libleptonica-dev",
            "apt-get install -y tesseract-ocr libtesseract-dev",
        ];

        for cmd in commands.iter() {
            let status = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status()
                .expect(&format!("Failed to run: {}", cmd));

            if !status.success() {
                panic!("Failed to run: {}", cmd);
            }
        }
    }

    // Tell cargo to look for libraries in the standard system locations
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search=native=/usr/lib");
    
    // Set pkg-config path
    std::env::set_var(
        "PKG_CONFIG_PATH",
        "/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig",
    );
} 