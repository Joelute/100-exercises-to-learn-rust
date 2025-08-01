// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::{thread};

pub fn sum(v: Vec<i32>) -> i32 {
    let mid_point = v.len()/2;

    let mut first_half_vec = Vec::new();
    let mut second_half_vec = Vec::new();

    for (index,value) in v.iter().enumerate(){
        if index <= mid_point {
            first_half_vec.push(*value);
        } else {
            second_half_vec.push(*value);
        }
    }

    let first_half = thread::spawn(move||{
        first_half_vec.iter().sum()
    });

    let second_half = thread::spawn(move||{
        second_half_vec.iter().sum()
    });

    let first_half_sum: i32 = first_half.join().unwrap();
    let second_half_sum: i32 = second_half.join().unwrap();

    first_half_sum + second_half_sum
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
