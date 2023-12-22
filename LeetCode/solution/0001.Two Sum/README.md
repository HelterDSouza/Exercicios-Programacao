# [1. Two Sum](https://leetcode.com/problems/two-sum)

\# Easy

## Description

Given an array of integers `nums` and an integer `target`, return *indices of the two numbers such that they add up to `target`*.

You may assume that each input would have ***exactly* one solution**, and you may not use the *same* element twice.

You can return the answer in any order.

**Example 1:**

**Input:** nums = [2,7,11,15], target = 9
**Output:** [0,1]
**Explanation:** Because nums[0] + nums[1] == 9, we return [0, 1].

**Example 2:**

**Input:** nums = [3,2,4], target = 6
**Output:** [1,2]

**Example 3:**

**Input:** nums = [3,3], target = 6
**Output:** [0,1]

**Constraints:**

- `2 <= nums.length <= 10^4^`
- `-10^9^ <= nums[i] <= 10^9^`
- `-10^9^ <= target <= 10^9^`
- **Only one valid answer exists.**

**Follow-up:**Can you come up with an algorithm that is less than `O(n^2^)`time complexity?

## Solutions

### Rust

```rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hashmap: HashMap<i32, i32> = HashMap::new();

        for (index, value) in nums.iter().enumerate() {
            match hashmap.get(&(target - *value)) {
                Some(&index2) => return [index2, index as i32].to_vec(),
                None => hashmap.insert(*value, index as i32),
            };
        }
        panic!()
    }
}

```

### Python

```python
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hashmap:dict[int,int] = {}

        for index, value in enumerate(nums):
            
            complement: int = target - value
            
            if complement in hashmap:
                return [hashmap[complement], index]
                
            hashmap[value] = index
```
