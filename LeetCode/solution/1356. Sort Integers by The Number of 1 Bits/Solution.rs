struct Solution {}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by_key(|&x| (x.count_ones(), x));
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_case() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            [0, 1, 2, 4, 8, 3, 5, 6, 7].to_vec(),
            Solution::sort_by_bits(nums)
        )
    }
    #[test]

    fn test_second_case() {
        let nums = vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        assert_eq!(
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024].to_vec(),
            Solution::sort_by_bits(nums, target)
        )
    }
}
