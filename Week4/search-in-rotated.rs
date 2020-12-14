struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut size = nums.len();
        if size == 0 {
            return -1;
        }
        let mut base = 0_usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            if nums[mid] == target {
                return mid as i32;
            }
            if !(((nums[base] < nums[mid]) && (target >= nums[base] && target <= nums[mid]))
                || ((nums[base] > nums[mid]) && (target >= nums[base] || target <= nums[mid])))
            {
                base = mid;
            }
            size -= half;
        }
        if nums[base] == target {
            base as i32
        } else {
            -1
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_01() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_search_02() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_search_03() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
