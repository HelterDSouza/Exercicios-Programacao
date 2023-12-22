from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hashmap:dict[int,int] = {}

        for index, value in enumerate(nums):
            
            complement: int = target - value
            
            if complement in hashmap:
                return [hashmap[complement], index]
                
            hashmap[value] = index


class Test:
    def test_first_case(self):
        nums = [2, 7, 11, 15]
        target = 9
        assert  [0, 1] == Solution().twoSum(nums, target)

    def test_second_case(self):
        nums = [3, 2, 4]
        target = 6
        assert [1, 2] == Solution().twoSum(nums, target)

    def test_third_case(self):
        nums = [3, 3]
        target = 6
        assert [0, 1] == Solution().twoSum(nums, target)
