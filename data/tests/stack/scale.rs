use super::Stack;

pub fn scale(mut num: u64, base: u64) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '6', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut stack = Stack::new();

    while num != 0 {
        stack.push(num % base);
        num = num / base;
    }

    let mut num = String::new();

    while !stack.is_empty() {
        num.push(digits[stack.pop().unwrap() as usize])
    }

    num
}
