
use super::context::{METHOD, Context};

pub trait Router {
    fn get(&self, pat: &'static str, ctx: Context) {
        self.add(METHOD::GET, pat, ctx);
    }
    fn post(&self, pat: &'static str, ctx: Context) {
        self.add(METHOD::POST, pat, ctx);
    }
    fn put(&self, pat: &'static str, ctx: Context) {
        self.add(METHOD::PUT, pat, ctx);
    }
    fn patch(&self, pat: &'static str, ctx: Context) {
        self.add(METHOD::PATCH, pat, ctx);
    }
    fn delete(&self, pat: &'static str, ctx: Context) {
        self.add(METHOD::DELETE, pat, ctx);
    }
    fn add(&self, mth: METHOD, pat: &'static str, ctx: Context);
}
