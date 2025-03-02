use std::io::{stdout, Write};

/// get user's input. return String.
///
/// ## how to get input without message.
/// ```
/// let input = get_input("");
/// ```
///
/// ## how to get input with message.
/// ```
/// let input = get_input("please title>");
/// ```
///
/// ```bash
/// // 👇console
/// please title>foo 👈foo is user's input.
/// // input == "foo"
/// ```
pub fn get_input<S: AsRef<str>>(message: S) -> String {
    print!("{}>", message.as_ref());
    stdout().flush().unwrap();
    let mut word = String::new();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    word.trim().to_string()
}

/// if user's input is "y" or "yes", return true.
pub fn yes_no<S: AsRef<str>>(message: S) -> bool {
    let input = get_input(message.as_ref());
    input.is_empty() || matches!(input.as_ref(), "y" | "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        assert_eq!(get_input("hello?>"), "hoge".to_string());
    }

    #[test]
    fn yes_no_test() {
        println!("{}", yes_no("message"));
    }
}
