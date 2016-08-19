extern crate aries;

use aries::config::Loader;

#[test]
fn test_config_toml() {
    let file = "tests/config.toml";
    let name = "httpd";
    let port = 8080;
    let tmp = aries::config::Httpd {
        secrets: "change-me".to_string(),
        port: port,
        host: "localhost".to_string(),
    };

    let mut loader = aries::config::toml::Loader::new();
    loader.set(name, tmp).unwrap();
    let tmp1: aries::config::Httpd = loader.get(name).unwrap();
    loader.write(file).unwrap();
    loader.read(file).unwrap();
    assert_eq!(port, tmp1.port);


}
