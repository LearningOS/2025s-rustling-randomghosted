// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut left=0; let mut right =input.len();
    let chars: Vec<char>= input.chars().collect();
    for i in 0..chars.len()-1{
        if chars[i]==' '{
            
        }else{
            left=i;
            break;
        }
    }

    for i in (0..input.len()).rev(){
        if chars[i]==' '{
            
        }else{
            right=i+1;
            break;
        }
    }
    println!("{}",right);
    println!("{}",left);
    chars[left..right].iter().collect::<String>()
}

fn compose_me(chars: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut s= chars.to_string();
    s.push_str(" world!");
    s
}

fn replace_me(chars: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    chars.to_string().replace("cars","balloons")
}

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
