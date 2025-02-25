// heap_sort

macro_rules! parent {
    ($child: ident) => {
        $child >> 1
    };
}

macro_rules! left_child {
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}

macro_rules! right_child {
    ($parent:ident) => {
        ($parent + 1) << 1
    };
}

fn  heap_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {return;}

    let len = nums.len();
    let last_parent = parent!(len);
    for i in (0..=last_parent).rev() {
        move_down(nums, i);
    }

    for end in (1..nums.len()).rev() {
        nums.swap(0, end);
        move_down(&mut nums[..end], 0);
    }
}

fn move_down(nums: &mut [i32], mut parent: usize) {
    let last = nums.len() - 1;
    loop {
        let left = left_child!(parent);
        let right = right_child!(parent);

        if left > last {break;}

        let child = if right <= last && nums[left] < nums[right] {
            right
        } else {
            left
        };

        if nums[child] > nums[parent] {
            nums.swap(parent, child);
        }

        parent = child;
    }
}

#[cfg(test)]
mod test {
    use crate::random_vec;

    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut nums = random_vec(30, 100, 0);
        heap_sort(&mut nums);
        println!("after sort::{:?}", nums);
    }
}
