#[test]
fn ui() {
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
