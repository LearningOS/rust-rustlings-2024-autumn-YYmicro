// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };// 宏的定义可以有多个参数，多个定义加上分号更安全
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
