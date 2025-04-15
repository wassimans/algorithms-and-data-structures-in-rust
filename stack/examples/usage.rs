use stack::Stack;

fn main() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    println!("Top: {:?}", s.peek());
}
