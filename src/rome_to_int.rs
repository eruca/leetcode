pub fn roman_to_int(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut peekable = s.as_bytes().into_iter().peekable();

    let mut sum = 0;
    loop {
        let curr_char = peekable.next().unwrap();

        let next_char = match peekable.peek() {
            Some(&c) => c,
            _ => return sum + get_int(*curr_char),
        };

        let curr_int = get_int(*curr_char);
        if curr_int < get_int(*next_char) {
            sum += -1 * curr_int;
        } else {
            sum += curr_int;
        }
    }
}

fn get_int(c: u8) -> i32 {
    match c {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => panic!("unknown {c}"),
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994)
    }
}
