# lua-macro
Have you ever wanted to embed Lua code straight into Rust?  

No?  

Well, you can now lol

## How to use
Import `lua_macro::lua` and use `lua! { your code }` to insert some Lua.
To evaluate an expression, use `lua_macro::lua_eval!`.