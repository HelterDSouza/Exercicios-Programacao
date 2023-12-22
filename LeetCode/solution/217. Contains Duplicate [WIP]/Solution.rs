struct Solution {}

impl Solution {
    pub fn contains_duplicate(mut arr: Vec<i32>) -> Vec<i32> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_case() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(true, Solution::contains_duplicate(nums))
    }
    #[test]

    fn test_second_case() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(false, Solution::contains_duplicate(nums, target))
    }
    #[test]

    fn test_third_case() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(true, Solution::contains_duplicate(nums, target))
    }
}
