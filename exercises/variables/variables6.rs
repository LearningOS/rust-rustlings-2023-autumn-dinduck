// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

// 全局的变量不能自动推断类型, let 实际是 Rust 的语法糖, 这里必须标注变量类型, static 和 const
// 由于声明全局变量
// 其中 *const 表示 &T , *mut &mut T, 这是 Rust 的地址指针
const NUMBER: i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
