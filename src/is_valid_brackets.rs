fn is_valid(s: String) -> bool {
    let chars = s.as_bytes();
    let mut v: Vec<u8> = Vec::new();

    for i in 0..chars.len() {
        match chars[i] {
            c @ b'(' | c @ b'[' | c @ b'{' => v.push(c),
            b')' => {
                if !pop(&mut v, b'(') {
                    return false;
                }
            }
            b']' => match v.pop() {
                Some(b) if b == b'[' => {}
                _ => return false,
            },
            b'}' => match v.pop() {
                Some(b) if b == b'{' => {}
                _ => return false,
            },
            _ => {}
        }
    }

    v.len() == 0
}

fn pop(v: &mut Vec<u8>, b: u8) -> bool {
    let c = match v.pop() {
        Some(c) => c,
        _ => return false,
    };
    c == b
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_valid("()".into()), true);
        assert_eq!(is_valid("[".into()), false);
    }
}
