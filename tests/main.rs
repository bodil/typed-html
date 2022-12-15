fn main() {
    println!("This crate is not meant to be run, it only serves as a compilation test.");
}

#[test]
fn ui() {
    extern crate version_check;

    if !version_check::is_feature_flaggable().unwrap_or(false) {
        return;
    }

    extern crate compiletest_rs as compiletest;

    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::Ui,
        src_base: std::path::PathBuf::from("cases"),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}
