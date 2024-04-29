// LeetCode 4. Median of Two Sorted Arrays

//Given two sorted arrays nums1 and nums2 of 
//size m and n respectively, return the median 
//of the two sorted arrays.

//The overall run time complexity should be O(log (m+n)).

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut a, mut b) = (nums1, nums2);
        let total = a.len() + b.len();
        let half = total / 2;

        if b.len() < a.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let mut l = 0;
        let mut r = a.len() as i32 - 1;
        loop {
            let i = (l + r) / 2;
            let j = half as i32 - i - 2;

            let aleft = if i >= 0 { a[i as usize] as f64 } else { f64::NEG_INFINITY };
            let aright = if (i + 1) < a.len() as i32 { a[(i + 1) as usize] as f64 } else { f64::INFINITY };
            let bleft = if j >= 0 { b[j as usize] as f64 } else { f64::NEG_INFINITY };
            let bright = if (j + 1) < b.len() as i32 { b[(j + 1) as usize] as f64 } else { f64::INFINITY };

            if aleft <= bright && bleft <= aright {
                if total % 2 != 0 {
                    return aright.min(bright);
                } else {
                    return (aleft.max(bleft) + aright.min(bright)) / 2.0;
                }
            } else if aleft > bright {
                r = i - 1;
            } else {
                l = i + 1;
            }
        }
    }
}

fn main(){
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays_example1() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    }

    #[test]
    fn test_find_median_sorted_arrays_example2() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
