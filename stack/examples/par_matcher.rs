use stack::Stack;

fn main() {}

fn par_matcher(chars: &str) -> bool {
    let mut s = Stack::new();

    for ch in chars.chars() {
        if matches!(ch, '(') {
            s.push(ch);
        } else if matches!(ch, ')') {
            // If the stack is empty, nothing to match
            if s.is_empty() {
                return false;
            }

            let top_ch = s.pop();
            if !matches!(top_ch, Some('(')) {
                return false;
            }
        }
    }

    s.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn balanced() {
        assert!(par_matcher("()((()))"));
    }

    #[test]
    fn unbalanced() {
        assert!(!par_matcher("()((())))"));
    }

    #[test]
    fn empty_is_balanced() {
        assert!(par_matcher(""));
    }

    #[test]
    fn only_opening() {
        assert!(!par_matcher("((("));
    }

    #[test]
    fn only_closing() {
        assert!(!par_matcher(")))"));
    }

    #[test]
    fn alternated_unbalanced() {
        assert!(!par_matcher("(()))(()"));
    }
}
