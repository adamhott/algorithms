// LeetCode 121. Best Time to Buy and Sell Stock
//You are given an array prices where prices[i] is 
//the price of a given stock on the ith day.

//You want to maximize your profit by choosing a single 
//day to buy one stock and choosing a different day in 
//the future to sell that stock.

//Return the maximum profit you can achieve from this 
//transaction. If you cannot achieve any profit, return 0.

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut lowest = prices[0];
        
        for price in prices {
            if price < lowest {
                lowest = price;
            }
            res = res.max(price - lowest);
        }
        
        res
    }
}

fn main() {
    todo!{}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;
        let result = Solution::max_profit(prices);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        let prices = vec![7, 6, 4, 3, 1];
        let expected = 0;
        let result = Solution::max_profit(prices);
        assert_eq!(result, expected);
    }
}
