use std::vec;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn build_list_node(list: &[i32]) -> Option<Box<ListNode>> {
    list.iter().fold(None, |acc, &v| {
        Some(Box::new(ListNode { val: v, next: acc }))
    })
}

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut vec_l1 = vec![];
    while let Some(v1) = l1 {
        vec_l1.push(v1.val);
        l1 = v1.next;
    }

    let mut vec_l2 = vec![];
    while let Some(v2) = l2 {
        vec_l2.push(v2.val);
        l2 = v2.next;
    }
    let vec_l1_length = vec_l1.len();
    let vec_l2_length = vec_l2.len();

    let (mut max_vec, min_vec) = if vec_l1_length > vec_l2_length {
        (
            vec_l1,
            vec_l2
                .into_iter()
                .chain(std::iter::repeat(0).take(vec_l1_length - vec_l2_length))
                .collect(),
        )
    } else if vec_l1_length < vec_l2_length {
        (
            vec_l2,
            vec_l1
                .into_iter()
                .chain(std::iter::repeat(0).take(vec_l2_length - vec_l1_length))
                .collect(),
        )
    } else {
        (vec_l1, vec_l2)
    };

    let mut ret = 0;
    let mut tmp;
    for i in 0..min_vec.len() {
        tmp = min_vec[i] + max_vec[i] + ret;
        max_vec[i] = tmp % 10;
        ret = tmp / 10;
    }

    if ret > 0 {
        max_vec.push(ret);
    }

    let mut op = None;
    for v in max_vec.into_iter().rev() {
        op = Some(Box::new(ListNode { val: v, next: op }));
    }
    op
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            add_two_numbers(
                build_list_node(&[9, 9, 9, 9, 9, 9, 9]),
                build_list_node(&[9, 9, 9, 9])
            ),
            build_list_node(&[1, 0, 0, 0, 9, 9, 9, 8])
        );
    }
}
