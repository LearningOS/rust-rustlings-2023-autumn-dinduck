// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
//  这里没有改变内部的数据， 直接借用即可
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// 这里对内部进行了修改， 可选可变借用或所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
