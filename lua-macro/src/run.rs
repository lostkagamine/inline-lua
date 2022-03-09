use mlua::prelude::*;

#[allow(dead_code)]
pub fn run_lua<'a, R>(code: &str) -> R
    where R: for<'local> FromLuaMulti<'local> + Clone
{
    let lua = Lua::new();
    let res: Result<R, _> = lua.load(code).eval();
    res.unwrap().clone()
}