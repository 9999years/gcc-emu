extern crate regex;

use std::process::Command;
use regex::Regex;

fn man(cmd: &str) -> String {
    String::from_utf8_lossy(&Command::new("man")
        .args(&["-Tutf8", "--nh", cmd])
        .output()
        .expect(&format!("Failed to find man page for {}", cmd))
        .stdout).into_owned();
}

fn args(cmd: &str) {
    println!("{}", man(cmd));
    let arg_re = Regex::new("\b[[:alnum:]-]\b");
}

fn main() {
    args("gcc");
}
