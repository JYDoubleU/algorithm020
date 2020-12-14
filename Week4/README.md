学习笔记

## 二分查找模板

```rust
let (mut left, mut right) = (0, len(array) - 1);
while left <= right: 
	  let mid = (left + right) / 2 
	  if array[mid] == target {
		    // find the target!! 
		    break or return result 
      } else if array[mid] < target  {
		    left = mid + 1;
      } else {
		    right = mid - 1;
      }
```