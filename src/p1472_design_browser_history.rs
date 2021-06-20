#[allow(dead_code)]
struct BrowserHistory {
    mem: Vec<String>,
    cur: usize,
}

#[allow(dead_code)]
impl BrowserHistory {
    
    fn new(homepage: String) -> Self {
        BrowserHistory {
            mem: vec![homepage],
            cur: 0
        }
    }

    fn visit(&mut self, url: String) {
        self.mem = self.mem[..=self.cur].to_vec();
        self.mem.push(url);
        self.cur += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.cur -= (steps as usize).min(self.cur);
        self.mem[self.cur].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.cur += (steps as usize).min(self.mem.len()-1 - self.cur);
        self.mem[self.cur].clone()
    }
}

#[test]
fn example() {
    let mut bh = BrowserHistory::new("leetcode.com".to_string());
    assert_eq!(bh.visit("google.com".to_string()), ());
    assert_eq!(bh.visit("facebook.com".to_string()), ());
    assert_eq!(bh.visit("youtube.com".to_string()), ());
    assert_eq!(bh.back(1), "facebook.com".to_string());
    assert_eq!(bh.back(1), "google.com".to_string());
    assert_eq!(bh.forward(1), "facebook.com".to_string());
    assert_eq!(bh.visit("linkedin.com".to_string()), ());
    assert_eq!(bh.forward(2), "linkedin.com".to_string());
    assert_eq!(bh.back(2), "google.com".to_string());
    assert_eq!(bh.back(7), "leetcode.com".to_string());
}