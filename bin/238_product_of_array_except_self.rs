// Leetcode 238. Product of Array Except Self
// Given an integer array nums, return an array 
// answer such that answer[i] is equal to the 
// product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is 
// guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time 
// and without using the division operation.

struct Solution;

impl Solution {
    fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];

        // Calculate prefix products
        for i in 1..nums.len() {
            res[i] = res[i - 1] * nums[i - 1];
        }

        // Calculate postfix products and combine with prefix products
        let mut postfix = 1;
        for i in (0..nums.len()).rev() {
            res[i] *= postfix;
            postfix *= nums[i];
        }

        res
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_product_except_self_example1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), expected);
    }

    #[test]
    fn test_product_except_self_example2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];
        assert_eq!(Solution::product_except_self(nums), expected);
    }
}
