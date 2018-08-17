extern crate regex;
extern crate rand;

use std::process::Command;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::thread;
use std::time::Duration;

use rand::{thread_rng, Rng};
use regex::Regex;

const NM_PREFIX_LEN: usize = 19;
const NM_OFFSET_LEN: usize = 16;

fn fread(fname: &str) -> String {
    let mut f = File::open(fname)
        .expect(&format!("{} not found", fname));
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(&format!("Could not read {}", fname));
    contents
}

fn libfunctions(library: &str) -> Vec<String> {
    // nm output is like:
    // 0000000000000000 T atexit
    // |<---- 19 chars ->|
    Command::new("nm")
        .args(&["-p", library])
        .output()
        .expect(&format!("Failed to find library {}", library))
        .stdout
        .split(|byte| byte == &b'\n' || byte == &b'\r')
        // lines at least as long as the "prefix" (see above)
        .filter(|line| line.len() > NM_PREFIX_LEN)
        // chop off the hex offset
        .map(|line| &line[NM_OFFSET_LEN..])
        // T for text (code) section
        .filter(|line| line.starts_with(b" T "))
        // chop off section indicator & make String
        .map(|line| String::from_utf8_lossy(&line[3..]).into_owned())
        .collect()
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

fn gcc_invocation() -> String {
    invocation("gcc", 5..20)
}

// count: the max. number of args; args may be between the range's start and end
fn invocation(cmd: &str, count: Range<usize>) -> String {
    let args = args(cmd);
    if args.len() == 0 {
        return cmd.to_string();
    }

    let mut ret = Vec::new();
    ret.push(cmd);
    // avoid reallocations cause im sure THAT's a huge perf concern
    ret.reserve(count.end);
    let mut rng = thread_rng();
    for _ in 0..(rng.gen_range(count.start, count.end)) {
        ret.push(rng.choose(&args).unwrap().as_ref());
    }

    ret.join(" ")
}

fn compilation_time() -> Duration {
    Duration::new(0, thread_rng().gen_range(0, 3_000_000_000))
}

fn main() {
    //loop {
        //println!("{}", gcc_invocation());
        //thread::sleep(compilation_time());
    //}
    let fns = libfunctions("/lib/libc.a");
    for libfn in fns {
        println!("{}", libfn);
    }
}
