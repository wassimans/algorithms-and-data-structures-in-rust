use queue::Queue;

fn main() {
    let names = vec!["Alice", "Bob", "Charlie", "Diana", "Eve"];
    let pass_number: usize = 3;
    let (winner, _) = hot_potato_winner(names, pass_number);

    assert_eq!(winner, "Diana");
}

fn hot_potato_winner(names: Vec<&str>, pass_count: usize) -> (&str, Vec<&str>) {
    let mut q: Queue<&str> = Queue::new();

    for n in names {
        q.enqueue(n);
    }

    let mut eliminated = Vec::new();

    while q.len() > 1 {
        for _ in 0..pass_count - 1 {
            if let Some(name) = q.dequeue() {
                q.enqueue(name);
            };
        }
        if let Some(removed_name) = q.dequeue() {
            eliminated.push(removed_name);
            println!("🎯 {} is removed!", removed_name);
        }
    }

    let winner = q.peek().expect("Must there be a winner left");
    println!("🏆 {} is the winner", winner);
    (winner, eliminated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_potato_winner_basic() {
        let names = vec!["Alice", "Bob", "Charlie", "Diana", "Eve"];
        let (winner, eliminated) = hot_potato_winner(names, 3);

        assert_eq!(winner, "Diana");
        assert_eq!(eliminated, vec!["Charlie", "Alice", "Eve", "Bob"]);
    }

    #[test]
    fn test_single_participant() {
        let names = vec!["OnlyOne"];
        let (winner, eliminated) = hot_potato_winner(names, 3);

        assert_eq!(winner, "OnlyOne");
        assert!(eliminated.is_empty());
    }

    #[test]
    fn test_two_participants() {
        let names = vec!["Alice", "Bob"];
        let (winner, eliminated) = hot_potato_winner(names, 3);

        assert!(["Alice", "Bob"].contains(&winner));
        assert_eq!(eliminated.len(), 1);
        assert_ne!(eliminated[0], winner);
    }

    #[test]
    fn test_order_is_correct() {
        let names = vec!["A", "B", "C", "D"];
        let (winner, eliminated) = hot_potato_winner(names, 2);
        assert_eq!(eliminated.len(), 3);
        assert!(eliminated.iter().all(|&name| name != winner));
    }
}
