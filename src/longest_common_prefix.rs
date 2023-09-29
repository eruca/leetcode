pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut v = String::new();
    if strs.len() == 0 {
        return v;
    }

    for i in 0.. {
        let bs = strs[0].as_bytes();
        if bs.len() < i + 1 {
            return v;
        }
        let c = bs[i];
        for ii in 1..strs.len() {
            let bs = strs[ii].as_bytes();
            if bs.len() < i + 1 {
                return v;
            }
            if c != bs[i] {
                return v;
            }
        }
        v.push(c as char);
    }
    v
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            "fl".to_string(),
        );

        assert_eq!(longest_common_prefix(vec!["".into()]), "".to_string(),);
        assert_eq!(longest_common_prefix(vec!["a".into()]), "a".to_string(),);
        assert_eq!(
            longest_common_prefix(vec!["ab".into(), "a".into()]),
            "a".to_string(),
        );
    }
}
