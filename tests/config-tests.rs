extern crate aries;

use aries::config::Loader;

#[test]
fn test_config_toml() {
    let file = "tests/config.toml";
    let name = "httpd";
    let tmp = aries::config::Httpd {
        port: 8080,
        host: "localhost".to_string(),
    };
    aries::config::toml::Loader::write(file, name, tmp.clone());

    let cfg = aries::config::toml::Loader::read(file, name, tmp.clone())
        .expect("open toml config fail: ");
    println!("{:?}", cfg);
    assert_eq!(tmp.port, cfg.port);
    std::fs::remove_file(file).expect("error on remove ");

}
