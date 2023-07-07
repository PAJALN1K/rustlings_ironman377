// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let string_1 = input.trim_left().trim_right();
    let string_1 = String::from(string_1);

    string_1
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut string_2 = String::from(input);
    
    string_2.push_str(" world!");

    string_2
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let string_3 = input.replace("cars", "balloons");
    
    let string_3 = String::from(string_3);

    string_3
}

// fn convert_option(input: Option<usize>) -> usize {
//     if let Some(i) = input {
//         let &output = i;
//     };
//     output
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
