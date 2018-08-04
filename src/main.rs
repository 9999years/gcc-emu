extern crate regex;

use std::process::Command;
use std::collections::HashSet;
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
    let man_page = man(cmd);
    // collect arguments in the man page into a vec and return them
    let arg_set = arg_re.find_iter(&man_page).map(
        // hell
        |m| m.as_str()
        ).collect::<HashSet<&str>>();
    arg_set.iter()
        .map(|m| m.to_string().trim().to_string())
        .collect::<Vec<String>>()
}

//fn fake_invocation(cmd: &str, args: Vec<String>) {
//}

fn main() {
    let args = args("gcc");
    for arg in &args {
        println!("{}", arg);
    }
    println!("{} args", args.len());
}
