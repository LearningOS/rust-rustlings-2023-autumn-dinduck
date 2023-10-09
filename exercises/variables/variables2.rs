// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // 若只是声明变量 let x, rust会推导变量类型， 但是此时rust并不知道x的类型, let x: u32;
    // 这样rust能知道x是u32类型， 但是此时是 uninit, let x = 0; , let x = 0u; let x = 0i;
    // 这样rust可以推导x的类型
    let x = 0;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
