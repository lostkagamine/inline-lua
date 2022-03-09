mod macros;

use lua_macro::{lua, lua_eval};
use macros::it_must;

it_must! {
    run_some_lua_code {
        lua! {
            local a = 2 + 2
            assert(a == 4)
        }
        true
    }

    do_math_correctly {
        let a: i32 = lua_eval! {
            2 + 2
        };
        a == 4
    }
}