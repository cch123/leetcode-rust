struct Solution;
// Definition for an interval.
 #[derive(Debug, PartialEq, Eq)]
 pub struct Interval {
   pub start: i32,
   pub end: i32,
 }

 impl Interval {
   #[inline]
   pub fn new(start: i32, end: i32) -> Self {
     Interval {
       start,
       end
     }
   }
 }

impl Solution {
    pub fn merge(intervals: Vec<Interval>) -> Vec<Interval> {
        let mut intervals = intervals;
        intervals.sort_by(|x, y| x.start.cmp(&y.start));
        let mut res = vec![];
        for span in intervals {
            if res.len() == 0 {
                res.push(span);
                continue;
            }

            // can be merged with the top of the stack?
            if res[res.len() - 1].end >= span.start {
                let start = res[res.len() - 1].start.min(span.start);
                let end = res[res.len() - 1].end.max(span.end);
                res.pop();
                res.push(Interval::new(start, end));
            } else {
                res.push(span);
            }
        }

        return res;
    }

    pub fn insert(intervals: Vec<Interval>, new_interval: Interval) -> Vec<Interval> {
        let mut intervals = intervals;
        intervals.push(new_interval);
        return Solution::merge(intervals);
    }
}

fn main() {
    println!("Hello, world!");
}
