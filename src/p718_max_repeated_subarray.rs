//! 718. Maximum length of repeated subarray
//!
//! Given two integer arrays 'nums1' and 'nums2', return the maximum length
//! of a subarray which appears in both arrays

use std::collections::HashMap;
use std::cmp;

pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut mem: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, num) in nums1.iter().enumerate() {
        if let Some(vec) = mem.get_mut(num) {
            vec.push(i);
        } else {
           mem.insert(*num, vec![i]);
        }
    }

    let mut max = 0;
    for (i, num) in nums2.iter().enumerate() {
        if let Some(vec) = mem.get(num) {
            let mut local_max = 0;
            for j in vec {
                let mut n1 = nums1[i..].iter();
                let mut n2 = nums2[*j..].iter();
                while !(n1.len() == 0) && !(n2.len() == 0) && n1.next() == n2.next() {
                    local_max += 1;
                }
                max = cmp::max(max, local_max);
                local_max = 0;
            }
        }
    }

    max
}

#[test]
fn example_1() {
    let nums1 = vec![1, 2, 3, 2, 1];
    let nums2 = vec![3, 2, 1, 4, 7];
    assert_eq!(find_length(nums1, nums2), 3)
}

#[test]
fn example_2() {
    let nums1 = vec![0, 0, 0, 0, 0];
    let nums2 = vec![0, 0, 0, 0, 0];
    assert_eq!(find_length(nums1, nums2), 5)
}

#[test]
fn failed_1() {
    let nums1 = vec![0, 0, 0, 0, 1];
    let nums2 = vec![1, 0, 0, 0, 0];
    assert_eq!(find_length(nums1, nums2), 4)
}
