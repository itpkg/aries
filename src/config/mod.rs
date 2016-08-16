pub mod toml;


pub trait Config {
    fn string(&self, key: &str, def_val: &str) -> &str;
    fn int(&self, key: &str, def_val: i64) -> i64;
}
