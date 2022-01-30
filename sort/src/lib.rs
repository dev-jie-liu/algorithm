extern crate num_traits;

use num_traits::AsPrimitive;

// fn merge_sort<>

#[inline(always)]
fn merge_sort<T: 'static>(nums: &mut [T]) -> Vec<T>
    where
        T: num_traits::NumAssign + std::fmt::Display
        + std::cmp::PartialOrd + Copy + std::fmt::Debug,
        i32: AsPrimitive<T>
{
    if nums.len() == 1 {
        return nums.to_vec();
    }

    let nums_middle_len: usize = nums.len() / 2;
    let before_nums_order = merge_sort(&mut nums[..nums_middle_len]);
    let after_nums_order = merge_sort(&mut nums[nums_middle_len..]);

    let mut before_idx: usize = 0;
    let mut after_idx: usize = 0;
    let mut nums_order_idx: usize = 0;
    loop {
        if before_idx == before_nums_order.len() {
            for num in &after_nums_order[after_idx..] {
                nums[nums_order_idx] = *num;
                nums_order_idx += 1;
            }
            return nums.to_vec();
        }
        if after_idx == after_nums_order.len() {
            for num in &before_nums_order[before_idx..] {
                nums[nums_order_idx] = *num;
                nums_order_idx += 1;
            }

            return nums.to_vec();
        }

        if before_nums_order[before_idx] < after_nums_order[after_idx] {
            nums[nums_order_idx] = before_nums_order[before_idx];
            before_idx += 1;
        } else {
            nums[nums_order_idx] = after_nums_order[after_idx];
            after_idx += 1;
        }
        nums_order_idx += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = vec![3, 2, 4, 8, 1];
        let result = merge_sort(&mut input);
        println!("{:?}", result);
        assert_eq!(result, vec![1, 2, 3, 4, 8]);
    }
}
