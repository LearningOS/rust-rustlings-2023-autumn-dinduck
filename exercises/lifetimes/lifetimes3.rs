// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// 由于 &str 是借用过来的， 所以可能会出现, 原数据类型 drop， 这里并不知道&str 的生命周期
// 这里标注说明 'a 应该和 Book 一样长, 一般知道 &str 是 'static 的， 再次会缩短至少与 Book 一样长``
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
