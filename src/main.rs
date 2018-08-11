extern crate regex;
extern crate rand;

use std::process::Command;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

use rand::{thread_rng, Rng};
use regex::Regex;

fn fread(fname: &str) -> String {
    let mut f = File::open(fname)
        .expect(&format!("{} not found", fname));
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(&format!("Could not read {}", fname));
    contents
}

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

// count: the max. number of args; args may be between the range's start and end
fn gcc_invokation<T: AsRef<str>>(args: &[T], count: Range<usize>) -> String {
    if args.len() == 0 {
        return "gcc".to_string();
    }

    let mut ret = Vec::new();
    ret.push("gcc");
    ret.reserve(count.end);
    let mut rng = thread_rng();
    for _ in 0..(rng.gen_range(count.start, count.end)) {
        ret.push(rng.choose(args).unwrap().as_ref());
    }

    return ret.join(" ");
}

fn main() {
    let args = args("gcc");
    for _ in 0..50 {
        println!("{}", gcc_invokation(args.as_slice(), 5..20));
    }
}
