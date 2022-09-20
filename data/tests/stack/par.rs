static OPEN : &str = "(<{";
static CLOSE : &str = ")>}";

use data::stack::stack::Stack;


pub fn par(vec: String) -> bool {
    let mut stack = Stack::new();
    for i in vec.chars() {
        if i == '{' || i == '<' || i == '(' {
            stack.push(i)
        } else if i == '}' || i == '<' || i == ')'{
            if let Some(open)  = stack.pop(){
                if ! (OPEN.find(i) == CLOSE.find(open)) {
                    return false
                }
            }else{
                return false
            }
        }
    }
    true && stack.is_empty()
}