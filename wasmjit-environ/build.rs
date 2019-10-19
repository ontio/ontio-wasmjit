use std::process::Command;

fn main() {
    let git_rev = match Command::new("git").args(&["rev-parse", "HEAD"]).output() {
        Ok(output) => {
            let mut ver = String::from_utf8(output.stdout).unwrap();
            ver.truncate(8);
            ver
        }
        Err(_) => String::from("unknown"),
    };
    println!("cargo:rustc-env=GIT_REV={}", git_rev);
}
