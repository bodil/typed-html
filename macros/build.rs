extern crate lalrpop;
extern crate version_check;

fn main() {
    lalrpop::process_root().unwrap();

    if version_check::is_nightly().unwrap_or(false) {
        println!("cargo:rustc-cfg=can_join_spans");
        println!("cargo:rustc-cfg=can_show_location_of_runtime_parse_error");
    }
}
