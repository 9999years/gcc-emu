use std::process::Command;
use std::collections::HashSet;
use std::ops::Range;
use std::marker::PhantomData;
use std::borrow::Borrow;

use rand::{thread_rng, Rng};
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

#[derive(Debug)]
pub struct Arguments<'a> {
    cmd: String,
    args: Vec<String>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Arguments<'a> {
    pub fn new(cmd: &str) -> Arguments {
        Arguments {
            cmd: cmd.to_string(),
            args: args(cmd),
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for Arguments<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        thread_rng().choose(self.args.as_slice())
            .map(String::as_ref)
    }
}

fn gcc_invocation() -> String {
    invocation("gcc", 5..20)
}

// count: the max. number of args; args may be between the range's start and end
fn invocation(cmd: &str, count: Range<usize>) -> String {
    let args = Arguments::new(cmd);
    if args.args.len() == 0 {
        return cmd.to_string();
    }

    let mut ret = Vec::new();
    ret.push(cmd);
    // avoid reallocations cause im sure THAT's a huge perf concern
    ret.reserve(count.end);
    ret.extend(args
            .map(|a| a.borrow())
            .take(thread_rng().gen_range(count.start, count.end)));

    ret.join(" ")
}
