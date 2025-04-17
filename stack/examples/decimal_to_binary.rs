use stack::Stack;

fn main() {
    assert_eq!(decimal_to_binary(233), "11101001");
}

/// Converts a non-zero decimal (base-10) number into its binary (base-2) representation.
///
/// # Algorithm
///
/// This function uses a stack-based approach to convert a decimal number to binary:
/// - Repeatedly divide the input number by 2, pushing the remainder (`0` or `1`) onto a stack.
/// - After the number is reduced to 0, the binary representation is built by popping values off the stack,
///   ensuring the correct most-significant-bit-first order.
///
/// This mimics the classic "division-remainder" method for binary conversion.
///
/// # Panics
///
/// Panics if the input number is `0`, as the function expects a non-zero value.
///
/// # Arguments
///
/// * `n` - A `u32` decimal number (must be non-zero)
///
/// # Returns
///
/// A `String` containing the binary representation of the input number.
///
/// # Examples
///
/// ```rust
/// let binary = decimal_to_binary(13);
/// assert_eq!(binary, "1101");
/// ```
fn decimal_to_binary(n: u32) -> String {
    if n == 0 {
        panic!("Shoulb a number not equal to zero");
    }
    let mut s = Stack::new();
    let mut n = n;

    while n > 0 {
        s.push(n % 2);
        n /= 2;
    }

    let result: String = s
        .iter()
        .map(|t| char::from_digit(*t, 10).unwrap())
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_binary() {
        assert_eq!(decimal_to_binary(233), "11101001");
    }

    #[test]
    #[should_panic(expected = "Shoulb a number not equal to zero")]
    fn panics_when_number_is_zero() {
        decimal_to_binary(0);
    }

    #[test]
    fn converts_one_to_binary() {
        assert_eq!(decimal_to_binary(1), "1");
    }

    #[test]
    fn converts_1988_to_binary() {
        assert_eq!(decimal_to_binary(1988), "11111000100");
    }

    #[test]
    fn converts_large_number() {
        assert_eq!(decimal_to_binary(10_000), "10011100010000");
    }
}
