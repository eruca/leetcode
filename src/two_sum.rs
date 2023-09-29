use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut map = HashMap::new();

    for (i, &v) in nums.iter().enumerate() {
        let ret = target - v;

        match map.get(&ret) {
            Some(pre_idx) => {
                return vec![*pre_idx, i];
            }
            _ => {
                map.insert(v, i);
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum(vec![1, 3, 4, 2], 6), vec![2, 3]);
    }
}
