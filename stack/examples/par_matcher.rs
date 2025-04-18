use stack::Stack;

fn main() {
    assert!(par_matcher("()((()))"));
}

/// 1. Initialize an empty stack.
/// 2. Iterate through the string one character at a time.
/// 3. If the current character is an opening parenthesis, push it onto the stack.
/// 4. If the current character is a closing parenthesis:
///     • If the stack is empty → Invalid (nothing to match).
///     • Otherwise, pop the top item off the stack.
///     • Check if it matches the current closing parenthesis (e.g. ( with )).
///     • If it doesn’t match → Invalid.
/// 5. After the loop:
///     • If the stack is not empty, some parenthesis were left unclosed → Invalid.
///     • If the stack is empty, all parenthesis were matched and closed correctly → ✅ Valid.
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
        assert!(par_matcher("func(a+b)(((c*d)))"));
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
