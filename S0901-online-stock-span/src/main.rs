#[derive(Debug)]
struct StockSpanner {
    v: Vec<i32>,
}

/** You can modify the type of `self` for your need. */
impl StockSpanner {
    fn new() -> Self {
        return StockSpanner { v: vec![] };
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut l = self.v.len();
        let mut v = &mut self.v;
        if l == 0 {
            v.push(price);
            return 1;
        }
        //println!("fuck");
        let mut res = 1;
        //println!("l:{}", l);
        // 为什么注释掉的不行？
        // for i in (l-1)..0 {
        while l >0 {
            l -= 1;
            //println!("{}, v[i]: {}", l,v[l]);
            if v[l] <= price {
                res += 1;
            } else {
                break
            }
        }

        v.push(price);
        return res;
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

fn main() {
    let mut s = StockSpanner::new();
    println!("{}",s.next(100));
    println!("{}",s.next(80));
    println!("{}",s.next(60));
    println!("{}",s.next(70));
    println!("{}",s.next(60));
    println!("{}",s.next(75));
    println!("{}",s.next(85));
    println!("{:?}",s);
}
