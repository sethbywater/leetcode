struct ProductOfNumbers {
}

impl ProductOfNumbers {
    fn new() -> Self {

    }
    
    fn add(&mut self, num: i32) {
    }

    fn get_product(&self, k: i32) -> i32 {
    }
}

/// To make testing using leetcode's input format easier
#[cfg(test)]
fn evaluate(ops: Vec<&str>, inputs: Vec<i32>, correct_output: Vec<i32>) {
    let pon = ProductOfNumbers::new();
    for (i, op) in ops.enumerate() {
        match op {
            "ProductOfNumbers" => continue,
            "add" => pon.add(inputs[i]),
            "getProduct" => assert_eq!(pon.get_product(inputs[i]), correct_output[i]),
            _ => unreachable!()
        }
    }
}

#[test]
fn example() {
    let mut pon = ProductOfNumbers::new();

    pon.add(3);
    pon.add(0);
    pon.add(2);
    pon.add(5);
    pon.add(4);
    assert_eq!(pon.get_product(2), 20);
    assert_eq!(pon.get_product(3), 40);
    assert_eq!(pon.get_product(4), 0);
    pon.add(8);
    assert_eq!(pon.get_product(2), 32);
}
