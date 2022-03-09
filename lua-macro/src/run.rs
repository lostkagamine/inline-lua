use mlua::prelude::*;

#[allow(dead_code)]
pub fn run_lua(code: &str) {
    let lua = Lua::new();
    lua.load(code).exec().expect("lua error");
}