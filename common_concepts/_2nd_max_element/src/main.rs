fn second_largest(nums: &[i32]) -> Option<i32> {
    if nums.len() < 2 {
        return None;
    }
    let mut first = i32::MIN;
    let mut second = i32::MIN;
    for &num in nums {
        if num > first {
            second = first;
            first = num;
        } else if num > second && num != first {
            second = num;
        }
    }
    if second == i32::MIN {
        None
    } else {
        Some(second)
    }
}

fn second_max(nums: &[i32]) -> Option<i32> {
    let mut unique_nums = nums.to_vec();
    unique_nums.sort_unstable();
    unique_nums.dedup();
    if unique_nums.len() < 2 {
        None
    } else {
        Some(unique_nums[unique_nums.len() - 2])
    }
}

fn main() {
    println!("{:?}", second_largest(&[3, 5, 2, 9, 7]));
    println!("{:?}", second_largest(&[1, 1, 1]));
    println!("{:?}", second_max(&[3, 5, 2, 9, 7]));
    println!("{:?}", second_max(&[1, 1, 1]));
}
