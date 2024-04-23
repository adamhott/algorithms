// LeetCode 11. Container with Most Water

//You are given an integer array height of length n. There are n vertical lines
// drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

//Find two lines that together with the x-axis form a container, such that the 
//container contains the most water.

//Return the maximum amount of water a container can store.

//Notice that you may not slant the container.

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut res = 0;

        while l < r {
            let h = std::cmp::min(height[l], height[r]);
            let w = (r - l) as i32;
            res = std::cmp::max(res, h * w);

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        res
    }
}


fn main(){
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);
    }

    #[test]
    fn test_example_2() {
        let height = vec![1, 1];
        let result = Solution::max_area(height);
        assert_eq!(result, 1);
    }
}