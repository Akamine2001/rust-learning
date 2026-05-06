fn main() {
    let mut s = String::from("Hello World");
    print_length(&s);
    add_prefix(&mut s);
    append_word(s);
    // v append_wordに所有権が移ったためprintができずエラーが起きる
    // println!("{}", s)
}

fn print_length(s: &str) {
    let len = s.len();
    println!("{}", len);
}

fn add_prefix(s: &mut String) {
    s.insert_str(0, "[INFO]");
    println!("{}", s);
}

fn append_word(s: String) {
    let mut s = s;
    s.push_str(", add word");
    println!("{}", s);
}
