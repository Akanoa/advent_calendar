fn compliant_to_decreasing_rule(password: String) -> bool {
    let mut chars = password.chars();
    let mut current = match chars.next() {
        Some(c) => {
            match c.to_digit(10) {
                Some(d) => d,
                None => panic!("Unable to parse digit")
            }
        }
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
            return false;
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
            return true;
        }

        current = next;
    }

    false
}

fn compliant_to_strict_group_rule(password: String, number_of_element_by_group: i32) -> bool {

    let mut chars = password.chars().peekable();
    let mut checked_digit = match chars.next() {
        Some(c) => c,
        None => panic!("Empty password")
    };
    let mut occurrence_digit_checked = 1;

    loop {
        // check for a next digit
        match chars.next() {
            // there is a next digit
            Some(next) => {
                // the next digit is the same that the current one but we cn't return true yet
                // because this group can be more than a strict double group digit
                if next == checked_digit {
                    // because the next digit we can increase the counter
                    occurrence_digit_checked += 1;
                    // we check the next digit after the next
                    match chars.peek() {
                        // the next digit wasn't the last one
                        Some(&next_next) => {
                            // we set the current one to the next one for the next iteration
                            checked_digit = next;
                            // of the next after next is the same as current it's a larger group than 2
                            // we continue to try to find some other strict digit group than the guessed
                            if next_next == next {
                                continue
                            }
                            // It's a strict double group of digit before the end of the string
                            if occurrence_digit_checked == number_of_element_by_group {
                                // Success :D
                                return true
                            }
                        },
                        // There is no more digit after the next one
                        None => {
                            // But the occurrence counter is 2
                            if occurrence_digit_checked == number_of_element_by_group {
                                // Success
                                return true
                            }
                        }
                    }
                }
                // the current and next digit are different,
                // so we begin a new potential strict double digit group
                // therefore we reset the occurrence counter
                checked_digit = next;
                occurrence_digit_checked = 1;
            },
            // There is no more digit after the current one
            None => {
                // It's a strict double group of digit before the end of the string
                if occurrence_digit_checked == 2 {
                    // Success
                    return true
                }
                break
            }
        };

    }
    false
}

pub fn part1() -> u32 {
    let mut count: u32 = 0;
    for password in 264793..803935 {
        let password_string = password.to_string();
        if !compliant_to_decreasing_rule(password_string.clone()) {
            continue;
        }
        if !compliant_to_double_rule(password_string.clone()) {
            continue;
        }
        count += 1;
    }

    count
}

pub fn part2() -> u32 {
    let mut count: u32 = 0;
    for password in 264793..803935 {
        let password_string = password.to_string();
        if !compliant_to_decreasing_rule(password_string.clone()) {
            continue;
        }
        if !compliant_to_double_rule(password_string.clone()) {
            continue;
        }
        if !compliant_to_strict_group_rule(password_string.clone(), 2) {
            continue;
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{compliant_to_decreasing_rule, compliant_to_double_rule, compliant_to_strict_group_rule};

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
        assert_eq!(compliant_to_strict_group_rule("1".to_string(), 2), false, "not compliant to the only double rule only one '1' then end of string");
        assert_eq!(compliant_to_strict_group_rule("11".to_string(), 2), true, "compliant to the only double rule at the end of string");
        assert_eq!(compliant_to_strict_group_rule("111".to_string(), 2), false, "not compliant to the only double rule there is three '1' in row");
        assert_eq!(compliant_to_strict_group_rule("1111".to_string(), 2), false, "not compliant to the only double rule there is four '1' in row");
        assert_eq!(compliant_to_strict_group_rule("11111".to_string(), 2), false, "not compliant to the only double rule there is five '1' in row");
        assert_eq!(compliant_to_strict_group_rule("111111".to_string(), 2), false, "not compliant to the only double rule there is six '1' in row");
        assert_eq!(compliant_to_strict_group_rule("113311".to_string(), 2), true, "1 - compliant to rule there is a least one strict double");
        assert_eq!(compliant_to_strict_group_rule("113334".to_string(), 2), true, "2 - compliant to rule there is a least one double, there two '1' at the beginning");
        assert_eq!(compliant_to_strict_group_rule("123334".to_string(), 2), false, "not compliant to strict double rule, there isn't a strict double digit");
        assert_eq!(compliant_to_strict_group_rule("112233".to_string(), 2), true, "compliant to the strict double rule, there is a double '2' in the middle of string");
        assert_eq!(compliant_to_strict_group_rule("123444".to_string(), 2), false, "not compliant to strict double rule, there three '4' at the en ");
        assert_eq!(compliant_to_strict_group_rule("111122".to_string(), 2), true, "compliant to double rule, there is a strict double '2' a the end of string");
    }
}
