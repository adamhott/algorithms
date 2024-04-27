// LeetCode 875. Koko Eating Bananas

//Koko loves to eat bananas. There are n piles of bananas, 
//the ith pile has piles[i] bananas. The guards have gone 
//and will come back in h hours.

//Koko can decide her bananas-per-hour eating speed of k. 
//Each hour, she chooses some pile of bananas and eats k bananas 
//from that pile. If the pile has less than k bananas, she eats 
//all of them instead and will not eat any more bananas during this hour.

//Koko likes to eat slowly but still wants to finish eating all 
//the bananas before the guards return.

//Return the minimum integer k such that she can eat all the 
//bananas within h hours.

use std::f64;

struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1; // The lowest possible eating speed
        let mut r = *piles.iter().max().unwrap(); // The maximum pile value
        let mut res = r; // Default result

        while l <= r {
            let k = (l + r) / 2; // Midpoint for binary search
            let mut total_time = 0;

            // Calculate the total time required for eating at speed k
            for &p in &piles {
                // Using integer arithmetic for safe division and rounding
                total_time += (p + k - 1) / k; // This is equivalent to math.ceil(p / k)
            }

            if total_time <= h {
                res = k; // Update the result if within time limit
                r = k - 1; // Narrow the search range by adjusting the upper bound
            } else {
                l = k + 1; // Narrow the search range by adjusting the lower bound
            }
        }

        res // Return the minimum eating speed
    }
}


fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_eating_speed_example1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        let expected_output = 4;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, expected_output, "Test failed for example 1");
    }

    #[test]
    fn test_min_eating_speed_example2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        let expected_output = 30;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, expected_output, "Test failed for example 2");
    }

    #[test]
    fn test_min_eating_speed_example3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        let expected_output = 23;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, expected_output, "Test failed for example 3");
    }
}

