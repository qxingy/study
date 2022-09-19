use data::stack::stack::Stack;

fn stack_help(vec: String) -> bool {
    let mut stack = Stack::new();
    for i in vec.chars() {
        if i == '{' || i == '<' || i == '(' {
            stack.push(i)
        } else {
            if let Some(i1) = stack.peek() {
                if *i1 == i {
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
    true
}

#[test]
fn stack_test() {
    assert_eq!(stack_help("(((()){}{})){}{{{}>>}}".to_owned()), true);
    assert_eq!(stack_help("(((()){}{})){}{{}>>}}".to_owned()), false);
}
