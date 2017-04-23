use std::fmt;

fn main() {
    for num in 1i64..100 {
        if div_by_three(num) {
            print!("Fizz");
        }
        if div_by_five(num) {
            print!("Buzz");
        }

        print!("- {}\n", num);
    }
}


fn div_by_three(n: i64) -> bool {
    n % 3 == 0
}

#[test]
fn test_div_by_three() {
    assert!(!div_by_three(1), "One is not three");
    assert!(div_by_three(3), "three is three");
}

fn div_by_five(n: i64) -> bool {
    n % 5 == 0
}

#[test]
fn test_div_by_five() {
    assert!(!div_by_five(1), "One is not five");
    assert!(!div_by_five(3), "three is not five");
    assert!(div_by_five(5), "five is five");
    assert!(div_by_five(15), "fifteen is five");
}
