/*
 * @Author: LiuJie
 * @Date: 2022-01-25 13:48:14
 * @LastEditTime: 2022-01-26 09:28:15
 * @LastEditors: LiuJie
 */
use brunch::Bench;
use std::time::Duration;
use max_sub_array::max_sub_array;



brunch::benches!(
    Bench::new("max_sub_array", "max_sub_array")
    .timed(Duration::from_secs(3))
    .with(|| max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]))
);
