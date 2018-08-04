extern crate regex;

use std::process::Command;
use regex::Regex;

fn man(cmd: &str) -> String {
    String::from_utf8_lossy(&Command::new("man")
        .args(&["-Tutf8", "--nh", cmd])
        .output()
        .expect(&format!("Failed to find man page for {}", cmd))
        .stdout).into_owned()
}

// returns arguments listed in the man page for `cmd`
fn args(cmd: &str) -> Vec<String> {
    // an argument is space, a dash, alphanumerics, and space
    let arg_re = Regex::new("\\s-[[:alnum:]-]+\\s").unwrap();
    // get the man page text
    let man_page: String = man(cmd);
    // collect arguments in the man page into a vec and return them
    arg_re.find_iter(&man_page).map(
        |m| String::from(m.as_str()).trim().to_string()
        ).collect::<Vec<String>>()
}

fn main() {
    for arg in args("gcc") {
        println!("{}", arg);
    }
}
