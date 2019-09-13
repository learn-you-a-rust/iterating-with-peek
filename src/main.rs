fn main() {
    let s = "abbcccd";
    let mut iter = s.chars().peekable();
    while let Some(chr) = iter.next() {
        println!("current: {}", chr);
        let peek = *iter.peek().unwrap();
        if chr == peek {
            println!("peek: {}\n", peek);
        } else {
            println!("not same, advancing\n");
        }
    }
}
