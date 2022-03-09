use lua_macro::{lua, lua_eval};

fn main() {
    println!("hi from rust");
    lua! {
        function say_hi()
            print("and hi from lua!")
        end

        say_hi()
    };

    let a: i32 = lua_eval! {
        2 + 2
    };
    
    println!("{a}");
}
