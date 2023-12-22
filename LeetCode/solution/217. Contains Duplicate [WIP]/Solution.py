from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        return len(set(nums))!=len(nums)
        
class Test:
    def test_first_case(self):
        nums = [1,2,3,1]
        assert True is Solution().containsDuplicate(nums)

    def test_second_case(self):
        nums = [1,2,3,4]
        assert False is Solution().containsDuplicate(nums)
    def test_third_case(self):
        nums = [1,1,1,3,3,4,3,2,4,2]
        assert True is Solution().containsDuplicate(nums)
