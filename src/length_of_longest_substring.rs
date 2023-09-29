use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

fn length_of_longest_substring<S: AsRef<str>>(s: S) -> i32 {
    if s.as_ref() == "" {
        return 0;
    }
    let mut max = 1;
    let chars: Vec<(usize, char)> = s.as_ref().chars().enumerate().collect();
    let chars_length = chars.len();
    let mut map: HashMap<usize, HashSet<char>> = HashMap::with_capacity(chars_length);

    let mut to_remove = vec![];
    for (i, c) in chars {
        for (k, v) in (&mut map).into_iter() {
            if v.contains(&c) {
                to_remove.push(*k);
            } else {
                v.insert(c);
                if v.len() as i32 > max {
                    max = v.len() as i32;
                }
            }
        }

        while let Some(i) = to_remove.pop() {
            map.remove(&i);
        }

        if (chars_length - i) as i32 > max {
            let mut set = HashSet::new();
            set.insert(c);
            map.insert(i, set);
        }
    }

    max
}

fn index_of(slice: &[u8], b: &u8) -> Option<usize> {
    for (i, byte) in slice.into_iter().enumerate() {
        if byte == b {
            return Some(i);
        }
    }
    return None;
}

fn length_of_longest_substring2(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let bytes = s.as_bytes();

    let mut max: i32 = 0;
    let mut range = (0, 0);
    for (i, b) in bytes.into_iter().enumerate() {
        match index_of(&bytes[range.0..range.1], b) {
            Some(idx) => {
                range.0 = idx + 1;
                range.1 += 1
            }
            None => range.1 = i + 1,
        }
        println!("{:?}, max:{}", range, max);
        if (range.1 - range.0) as i32 >= max {
            max = (range.1 - range.0) as i32;
        }
    }

    max
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(length_of_longest_substring("bbbbb"), 1);
        assert_eq!(length_of_longest_substring("pwwkew"), 3);
        assert_eq!(length_of_longest_substring("au"), 2);
    }

    #[test]
    fn it_works2() {
        assert_eq!(length_of_longest_substring2("abcabcbb".into()), 3);
        // assert_eq!(length_of_longest_substring2("bbbbb".into()), 1);
        // assert_eq!(length_of_longest_substring2("pwwkew".into()), 3);
        // assert_eq!(length_of_longest_substring2("au".into()), 2);
    }
}
