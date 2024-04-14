// Leetcode 739. Daily Temperatures
// Given an array of integers temperatures
// represents the daily temperatures, return 
// an array answer such that answer[i] is the 
// number of days you have to wait after the ith day
//  to get a warmer temperature. If there is no 
// future day for which this is possible, keep 
// answer[i] == 0 instead.

struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = Vec::new();

        for (i, &t) in temperatures.iter().enumerate() {
            while let Some(&(stack_t, stack_i)) = stack.last() {
                if t > stack_t {
                    stack.pop();
                    res[stack_i] = (i - stack_i) as i32;
                } else {
                    break;
                }
            }
            stack.push((t, i));
        }

        res
    }
}


fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures_example_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }

    #[test]
    fn test_daily_temperatures_example_2() {
        let temperatures = vec![30, 40, 50, 60];
        let expected = vec![1, 1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }

    #[test]
    fn test_daily_temperatures_example_3() {
        let temperatures = vec![30, 60, 90];
        let expected = vec![1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }
}


