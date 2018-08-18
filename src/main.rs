extern crate regex;
extern crate rand;

use std::fs::File;
use std::io::Read;
//use std::thread;
use std::time::Duration;

use rand::{thread_rng, Rng};

mod configure;
mod cmds;

fn fread(fname: &str) -> String {
    let mut f = File::open(fname)
        .expect(&format!("{} not found", fname));
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(&format!("Could not read {}", fname));
    contents
}

fn compilation_time() -> Duration {
    Duration::new(0, thread_rng().gen_range(0, 3_000_000_000))
}

fn main() {
    //loop {
        //println!("{}", gcc_invocation());
        //thread::sleep(compilation_time());
    //}
    let fns = configure::libfunctions("/lib/libc.a");
    for libfn in fns {
        println!("{}", libfn);
    }
}
