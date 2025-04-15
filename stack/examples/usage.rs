use stack::stack::Stack;

fn main() {
    let mut s = Stack::new();
    s.push(42);
    println!("{:?}", s.peek());
}
