// Leetcode 853. Car Fleet
//There are n cars going to the same destination 
//along a one-lane road. The destination is target miles away.

//You are given two integer array position and speed, 
//both of length n, where position[i] is the position of the ith car and speed[i] is the speed of the ith car (in miles per hour).

//A car can never pass another car ahead of it, but 
//it can catch up to it and drive bumper to bumper at the same speed. The faster car will slow down to match the slower car's speed. The distance between these two cars is ignored (i.e., they are assumed to have the same position).

//A car fleet is some non-empty set of cars driving 
//at the same position and same speed. Note that a single
//car is also a car fleet.

//If a car catches up to a car fleet right at the destination 
//point, it will still be considered as one car fleet.

//Return the number of car fleets that will arrive 
//at the destination.

struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, f64)> = position.into_iter()
            .zip(speed.into_iter())
            .map(|(pos, spd)| (pos, (target - pos) as f64 / spd as f64))
            .collect();

        // Sort by position in descending order
        cars.sort_by(|a, b| b.0.cmp(&a.0));

        let mut stack: Vec<f64> = Vec::new();

        for &(_, time) in &cars {
            if let Some(&last_time) = stack.last() {
                if time > last_time {
                    stack.push(time);
                }
            } else {
                stack.push(time);
            }
        }

        stack.len() as i32
    }
}


fn main() {
    todo!();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_fleet_example1() {
        assert_eq!(Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
    }

    #[test]
    fn test_car_fleet_example2() {
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    }

    #[test]
    fn test_car_fleet_example3() {
        assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
    }
}
