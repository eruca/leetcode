pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut nums = vec![];

    let mut ret = x;
    while ret > 0 {
        nums.push(ret % 10);
        ret = ret / 10;
    }

    for i in 0..nums.len() {
        if nums[i] != nums[nums.len() - i - 1] {
            return false;
        }
    }
    true
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
    }
}
