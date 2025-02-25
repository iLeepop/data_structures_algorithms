// bucker_sort

use std::fmt::Debug;

struct Bucket<H, T> {
    hasher: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    fn new(hasher: H, value: T) -> Bucket<H, T> {
        Bucket {
            hasher,
            values: vec![value],
        }
    }
}

fn bucket_sort<H, T, F>(nums: &mut [T], hasher: F)
    where H: Ord, T: Ord + Clone + Debug, F: Fn(&T) -> H {
        let mut buckets: Vec<Bucket<H, T>> = Vec::new();

        for val in nums.iter() {
            let hasher = hasher(&val);

            match buckets.binary_search_by(|bct| bct.hasher.cmp(&hasher)) {
                Ok(idx) => buckets[idx].values.push(val.clone()),
                Err(idx) => buckets.insert(idx, Bucket::new(hasher, val.clone())),
            }
        }

        let ret = buckets.into_iter().flat_map(|mut bucket| {
            bucket.values.sort();
            bucket.values
        }).collect::<Vec<T>>();

        nums.clone_from_slice(&ret);
    }

#[cfg(test)]
mod tests {
    use crate::random_vec;

    use super::*;

    #[test]
    fn test_bucket_sort() {
        let mut nums = random_vec(20, 100, 0);
        bucket_sort(&mut nums, |x| x / 5);
        println!("after sort::{:?}", nums)
    }
}