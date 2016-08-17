extern crate aries;

#[test]
fn test_config_toml() {
    let cfg = aries::config::Configuration::from_toml("tests/config.toml")
        .expect("open toml config fail");
    println!("{:?}", cfg);
    // assert_eq!(8080, cfg.int("http.port", 8088));
    // assert_eq!(8088, cfg.int("http.port1", 8088));
}
