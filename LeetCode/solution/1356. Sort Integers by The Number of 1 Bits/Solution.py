from typing import List


class Solution:
    def sortByBits(self, arr: List[int]) -> List[int]:
        return sorted(arr, key=lambda x: (bin(x).count('1'), x))
    
    def sortByBitsBestTime(self, arr: List[int]) -> List[int]:
        arr.sort()
        arr.sort(key=int.bit_count)
        return arr
    
    def count_ones(self, string:int):
        binary = str(bin(string))[2:]
        return sum([int(x) for x in binary])
    
    def WorstSortByBits(self, arr: List[int]) -> List[int]:
        return sorted(arr,key= lambda num : (self.count_ones(num),num))

class Test:
    def test_first_case(self):
        nums = [0,1,2,3,4,5,6,7,8]
        assert [0,1,2,4,8,3,5,6,7] == Solution().twoSum(nums)

    def test_second_case(self):
        nums = [1024,512,256,128,64,32,16,8,4,2,1]
        assert [1,2,4,8,16,32,64,128,256,512,1024] == Solution().twoSum(nums)
