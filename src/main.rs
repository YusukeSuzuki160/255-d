use proconio::input;

use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Greater | Ordering::Equal => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }
    a.sort();
    let mut sum_list = Vec::new();
    sum_list.push(0 as u64);
    for i in 0..n {
        sum_list.push(sum_list[i] + a[i] as u64);
    }
    for i in x {
        let index = a.lower_bound(&i);
        let sum_a = sum_list[index];
        let sum_b = sum_list[n] - sum_a;
        //println!("{}", index);
        //println!("{}, {}", sum_a, sum_b);
        println!(
            "{}",
            sum_b as i128 + (i as i128 * (2 * index as i128 - n as i128)) as i128 - sum_a as i128
        );
    }
}
