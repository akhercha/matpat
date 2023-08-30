fn glob(pattern: &str, text: &str) -> bool {
    let mut pattern = pattern.chars().peekable();
    let mut text = text.chars().peekable();

    while pattern.peek().is_some() && text.peek().is_some() {
        match pattern.peek().unwrap() {
            '*' => {
                text.next();
                pattern.next();
            },
            '?' => panic!("Not implemented yet"),
            '[' => panic!("Not implemented yet"),
            _ => {
                if pattern.peek() != text.peek() {
                    return false;
                }
                text.next();
                pattern.next();
            },
        }
    }
    return pattern.peek().is_none() && text.peek().is_none();
}

fn main() {
    let res: bool = glob("Hello", "*ello");
    println!("{:?}", res);
}
