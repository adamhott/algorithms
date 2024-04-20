// Leetcode 84. Largest Rectangle in Histogram

//Given an array of integers heights representing 
//the histogram's bar height where the width of 
//each bar is 1, return the area of the largest 
//rectangle in the histogram.

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut stack: Vec<(usize, i32)> = Vec::new();

    for (i, &h) in heights.iter().enumerate() {
        let mut start = i;
        while let Some(&(index, height)) = stack.last() {
            if height > h {
                stack.pop();
                max_area = max_area.max(height * (i - index) as i32);
                start = index;
            } else {
                break;
            }
        }
        stack.push((start, h));
    }

    for &(i, h) in &stack {
        max_area = max_area.max(h * (heights.len() - i) as i32);
    }

    max_area
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area_example1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(largest_rectangle_area(heights), 10);
    }

    #[test]
    fn test_largest_rectangle_area_example2() {
        let heights = vec![2, 4];
        assert_eq!(largest_rectangle_area(heights), 4);
    }
}