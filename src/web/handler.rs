use super::context::Context;

pub trait Handler {
    fn call(ctx: Context);
}
