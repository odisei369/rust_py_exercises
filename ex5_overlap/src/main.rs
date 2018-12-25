use std::io::{self, Write};

fn main() {
    let a = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let overlap = sorted_overlap(&a, &b);
    println!("Overlap of {:?} and {:?}: {:?}", a, b, overlap);
}

fn sorted_overlap(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut a_iter = a.iter();
    let mut b_iter = b.iter();
    let mut left: Option<&i32> = a_iter.next();
    let mut right: Option<&i32> = b_iter.next();
    let mut overlap: Vec<i32> = Vec::new();
    loop {
        match (left, right) {
            (Some(a), Some(b)) => { 
                if a == b {
                    overlap.push(*a);
                    left = a_iter.next();
                    right = b_iter.next();
                } else if a < b {
                    left = a_iter.next();
                } else {
                    right = b_iter.next();
                }
             },
            (_, _) => { return overlap; },
        }
    }
}
