use std::process::Command;

extern crate tango;

fn main() { 
    tango::process_root().unwrap();
    let result = Command::new("pandoc")
        .args(&["-s", "-t", "revealjs", "src/*.md", "-o", "html/index.html", "--dump-args"])
        .status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
    println!("Pandoc exited with status: {}", result);
}
