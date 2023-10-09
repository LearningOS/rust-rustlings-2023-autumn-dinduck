// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // let mut t: Vec<_> = vec![];
    // let mut i = 0;
    // while i < 128 {
    //     t.push(i);
    //     i += 1;
    // }
    // let a = t;
    // 或者  , 这里可以自动推断
    let a: [u32; 1000] = [0; 1000];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
