/*
 * @Author:LiuJie
 * @Date: 2022-01-25 12:06:52
 * @LastEditors: LiuJie
 * @LastEditTime: 2022-01-26 09:36:57
 * @LastEditTime: 2022-01-25 14:43:19
 */

extern crate num_traits;

use crate::num_traits::AsPrimitive;


/// 扫描数组中第一个可能的和最大区间
///
/// # Arguments
///
/// * `nums`:
/// 数组切片
/// returns: (T, usize, usize, Option<&[T]>)
/// 最大和, 最大子区间起始索引，最大子区间终止索引，未扫描区间
/// # Examples
///
/// ```
///
/// ```
#[inline(always)]
fn first_max_sub_array<T: 'static>(nums: &[T]) -> (T, usize, usize, Option<&[T]>)
    where
        T: num_traits::NumAssign + std::fmt::Display + std::cmp::PartialOrd + Copy + std::fmt::Debug,
        i32: AsPrimitive<T>
{
    let nums_len = nums.len();

    //去除数组开头的连续小于等于0的数
    let mut left_index = 0;
    for num in nums {
        if *num > 0_i32.as_() {
            break;
        }
        left_index += 1;
    }

    //所有数都小于等于0
    if left_index == nums_len {
        let mut max_sum = nums[0];
        let mut max_index = 0;
        for (i, num) in nums.iter().enumerate() {
            if *num > max_sum {
                max_sum = *num;
                max_index = i;
            }
        }
        return (max_sum, max_index, max_index, None);
    }

    //去除数组结尾的连续小于等于0的数
    let mut right_index = nums_len - 1;
    for num in nums.to_vec().iter().rev() {
        if *num > 0_i32.as_() {
            break;
        }
        right_index -= 1;
    }

    let next_nums = &nums[left_index..right_index + 1];
    let mut max_sum = 0_i32.as_();
    let mut right_max_index = 0;
    let mut left_sum = 0_i32.as_();
    for (i, num) in next_nums.iter().enumerate() {
        left_sum += *num;
        if left_sum <= 0_i32.as_() {
            break;
        }

        if left_sum > max_sum {
            max_sum = left_sum;
            right_max_index = i;
        }
    }

    let next_begin_index = right_max_index + 1;
    let next_nums = &next_nums[next_begin_index..];

    return if next_nums.len() == 0 {
        (max_sum, left_index, right_max_index, None)
    } else {
        (max_sum, left_index, right_max_index, Some(&next_nums))
    };
}

#[inline(always)]
pub fn max_sub_array<T: std::cmp::PartialOrd + Copy + 'static>(nums: Vec<T>) -> T
    where
        T: num_traits::NumAssign + std::fmt::Display + std::fmt::Debug,
        i32: AsPrimitive<T>
{
    let mut first = true;
    let mut max_sum = 0_i32.as_();

    let mut input_nums = &nums[..];
    loop {
        let (sub_max_sum, _i_sub_begin, _i_sub_end, next_nums) =
            first_max_sub_array(input_nums);
        if first {
            max_sum = sub_max_sum;
            first = false;
        }

        if max_sum < sub_max_sum {
            max_sum = sub_max_sum;
        }

        match next_nums {
            Some(next) => {
                input_nums = next;
            }
            None => { break; }
        }
    }

    return max_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        let max_sum = max_sub_array_1(vec![-4, 8, 10, 5, -40, 20]);
        assert_eq!(max_sum, 21);
    }
}