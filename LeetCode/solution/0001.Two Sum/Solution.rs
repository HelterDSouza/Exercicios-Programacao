struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (index, value) in nums.iter().enumerate() {
            match hashmap.get(&(target - *value)) {
                Some(&index2) => return vec![index2, index as i32],
                None => hashmap.insert(*value, index as i32),
            };
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_case() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!([0, 1].to_vec(), Solution::two_sum(nums, target))
    }
    #[test]

    fn test_second_case() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!([1, 2].to_vec(), Solution::two_sum(nums, target))
    }
    #[test]
    fn test_third_case() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!([0, 1].to_vec(), Solution::two_sum(nums, target))
    }
}
