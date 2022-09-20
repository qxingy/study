
mod stack;

#[test]
fn stack_test() {
    assert_eq!(stack::par("(((()){}撒旦反抗拉萨酱豆腐{}飞洒)管理费的结果){}{{{}}}".to_owned()), true);
    assert_eq!(stack::scale(10,2),"1010");
    assert_eq!(stack::scale(10,3),"101");
}
