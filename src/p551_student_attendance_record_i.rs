//! # 551. Student attendance record 1
//!
//! You are given a string representing an attendance record for a
//! student where each character signifies whether the student was
//! absent, late, or present on thet day, represented with the
//! characters `'A'`, `'L'`, and `'P'`
//!
//! The student is eligible if they meet both of the following criteria:
//! * The student was absent for fewer that 2 days total
//! * The student was never late for more than 3 consecutive days
//!
//! Return a boolean indicating whether the student is eligible for an
//! attendance award

pub fn check_record(s: String) -> bool {
    let mut a = 0;
    let mut l = 0;
    for c in s.chars() {
        match c {
            'P' => {
                l = 0
            }
            'A' => {
                a += 1;
                if a >= 2 { return false }
                l = 0;
            }
            'L' => {
                l += 1;
                if l >= 3 { return false }
            }
            _ => unreachable!()
        }
    }
    true
}

#[test]
fn example_1() {
    assert_eq!(
        check_record("PPALLP".to_string()),
        true
    )
}

#[test]
fn example_2() {
    assert_eq!(
        check_record("PPALLL".to_string()),
        false
    )
}
