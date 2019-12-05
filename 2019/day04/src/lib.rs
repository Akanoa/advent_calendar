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

fn compliant_to_no_more_than_double(password: String) -> bool {

    println!("-------------");
    println!("password: {}", password );

    let mut chars = password.chars().peekable();
    let mut current = match chars.next() {
        Some(c) => c,
        None => panic!("Empty password")
    };

    println!("first digit {}", current);

    let mut result = false;


    loop {
        let next = match chars.next() {
            Some(c) => c,
            None => break
        };

        println!("next digit {}", next);

        if next == current  {

            let next_next = match chars.peek() {
                Some(c) => *c,
                None => return true
            };

            println!("next next digit {}", next_next);

            if next != next_next  {
                return true;
            } else {
                result = false;
            }
        }

        current = match chars.next() {
            Some(c) => c,
            None => break
        };

    }

    if !result {
        println!("Password {} not compliant", password);
    }

    result
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

pub fn part2() -> u32 {
    let mut count : u32 = 0;
    for password in 264793..803935 {
        let password_string = password.to_string();
        if !compliant_to_decreasing_rule(password_string.clone()) {
            continue;
        }
        if !compliant_to_double_rule(password_string.clone()) {
            continue;
        }
        if !compliant_to_no_more_than_double(password_string.clone()) {
            continue;
        }
        count+=1;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{compliant_to_decreasing_rule, compliant_to_double_rule, compliant_to_no_more_than_double};

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

    #[test]
    fn test_no_more_than_double() {

//        assert_eq!(false, compliant_to_no_more_than_double("111111".to_string()), "not compliant to double rule 1");
//        assert_eq!(true, compliant_to_no_more_than_double("113311".to_string()), "not compliant to double rule 1");
        assert_eq!(compliant_to_no_more_than_double("11".to_string()), true, "not compliant to double rule 1");
        assert_eq!(compliant_to_no_more_than_double("111".to_string()), false, "not compliant to double rule 1");
//        assert_eq!(true, compliant_to_no_more_than_double("113334".to_string()), "not compliant to double rule 1");
//        assert_eq!(true, compliant_to_no_more_than_double("112233".to_string()), "compliant to double rule");
//        assert_eq!(true, compliant_to_no_more_than_double("111122".to_string()), "compliant to double rule");
        assert_eq!(compliant_to_no_more_than_double("123444".to_string()), false, "not compliant to double rule 2");
    }
}
