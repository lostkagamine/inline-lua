use lua_macro::lua;

fn main() {
    println!("hi from rust");
    lua! {
        function say_hi()
            print("and hi from lua!")
        end

        say_hi()
    };
}
