struct BrowserHistory {
    mem: Vec<String>,
	cur: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
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

// 
// Your BrowserHistory object will be instantiated and called as such:
// let obj = BrowserHistory::new(homepage);
// obj.visit(url);
// let ret_2: String = obj.back(steps);
// let ret_3: String = obj.forward(steps);
// 