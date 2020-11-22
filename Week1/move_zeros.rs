fn main() {}

fn move_zeros(nums: &mut Vec<i32>) {
    // 记录下一个非零元素在最终数组中的位置
    let mut i = 0;
    while i < nums.len() && nums[i] != 0 {
        i += 1;
    }
    let mut j = i;
    while j < nums.len() {
        if nums[j] != 0 {
            nums.swap(i, j);
            i += 1;
        }
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::move_zeros;

    #[test]
    fn test_move_zeros() {
        let mut input = vec![0, 1, 2, 3, 0, 4, 0, 5];
        move_zeros(&mut input);
        println!("result = {:?}", input);
        assert_eq!(vec![1], input);
    }
}
