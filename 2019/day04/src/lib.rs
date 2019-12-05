fn compliant_to_decreasing_rule(password: String) -> bool{

    let mut chars = password.chars();
    let mut current = match chars.next() {
        Some(c) => {
            match c.to_digit(10) {
                Some(d) => d,
                None => panic!("Unable to parse digit")
            }
        },
        None => panic!("Empty password")
    };


    loop {
        let next = match chars.next() {
            Some(c) => match c.to_digit(10) {
                Some(d) => d,
                None => panic!("Unable to parse digit")
            },
            None => break
        };

        if next < current {
            return false
        }

        current = next;

     }

    true
}

fn compliant_to_double_rule(password: String) -> bool {

    let mut chars = password.chars();
    let mut current = match chars.next() {
        Some(c) => c,
        None => panic!("Empty password")
    };


    loop {
        let next = match chars.next() {
            Some(c) => c,
            None => break
        };

        if next == current {
            return true
        }

        current = next;

    }

    false
}

pub fn part1() -> u32 {
    let mut count : u32 = 0;
    for password in 264793..803935 {
        let password_string = password.to_string();
        if !compliant_to_decreasing_rule(password_string.clone()) {
            continue;
        }
        if !compliant_to_double_rule(password_string.clone()) {
            continue;
        }
        count+=1;
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{compliant_to_decreasing_rule, compliant_to_double_rule};

    #[test]
    fn test_double_rule() {
        assert_eq!(true, compliant_to_double_rule("111111".to_string()));
        assert_eq!(true, compliant_to_double_rule("122345".to_string()));
        assert_eq!(false, compliant_to_double_rule("123789".to_string()));
    }

    #[test]
    fn test_increasing_rule() {
        assert_eq!(true, compliant_to_decreasing_rule("111111".to_string()));
        assert_eq!(true, compliant_to_decreasing_rule("111123".to_string()));
        assert_eq!(true, compliant_to_decreasing_rule("135679".to_string()));
        assert_eq!(false, compliant_to_decreasing_rule("223450".to_string()));
    }
}