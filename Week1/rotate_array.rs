fn main() {}

pub fn rotate_ktimes(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    if len < 1 {
        return;
    }
    // 每次右移一步, 重复k次
    for _ in 0..k {
        let mut i = 0;
        let mut last = nums[0];
        while i < len - 1 {
            let tmp = nums[i + 1];
            nums[i + 1] = last;
            last = tmp;
            i += 1;
        }
        nums[0] = last;
    }
}

pub fn rotate_replace(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    if len < 1 {
        return;
    }
    // let mut count = 0;
    let mut start = 0;
    let end = len - k as usize;
    while start < end {
        let mut curIdx = start;
        let mut curVal = nums[curIdx];
        loop {
            let realIdx: usize = (curIdx + k as usize) % len;
            let tmp = nums[realIdx];
            nums[realIdx] = curVal;
            curVal = tmp;
            curIdx = realIdx;
            // count += 1;
            if curIdx == start {
                break;
            }
        }
        start += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let mut input = vec![-1, -100, 3, 99];
        let k = 2;
        let want = vec![3, 99, -1, -100];
        rotate_replace(&mut input, k);

        assert_eq!(input, want);
    }

    #[test]
    fn test_normal() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        let want = vec![5, 6, 7, 1, 2, 3, 4];
        rotate_ktimes(&mut input, k);

        assert_eq!(input, want);
    }
}
