use std::env;

fn main() {
    let key = "INTELMEDIASDKROOT";
    if let Ok(val) = env::var(key) {
        println!(r"cargo:rustc-link-search={}\lib\x64\", val);
    }
}
