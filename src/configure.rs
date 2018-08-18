use std::process::Command;

const NM_PREFIX_LEN: usize = 19;
const NM_OFFSET_LEN: usize = 16;

pub fn libfunctions(library: &str) -> Vec<String> {
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
        .filter(|line| ! line.starts_with("_"))
        .collect()
}
