#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    }
    if list2.is_none() {
        return list1;
    }

    let mut v: Vec<i32> = vec![];

    let mut v1 = (None, false);
    let mut v2 = (None, false);

    loop {
        if !v1.1 && v1.0.is_none() {
            v1 = match list1 {
                Some(ln) => {
                    list1 = ln.next;
                    (Some(ln.val), false)
                }
                None => (None, true),
            }
        }

        if !v2.1 && v2.0.is_none() {
            v2 = match list2 {
                Some(ln) => {
                    list2 = ln.next;
                    (Some(ln.val), false)
                }
                None => (None, true),
            }
        }

        if !v1.1 && !v2.1 {
            let v1v = v1.0.unwrap();
            let v2v = v2.0.unwrap();

            if v1v < v2v {
                v.push(v1v);
                v1.0 = None;
            } else if v1v > v2v {
                v.push(v2v);
                v2.0 = None;
            } else {
                v.push(v1v);
                v.push(v2v);
                v1.0 = None;
                v2.0 = None;
            }
        }

        if v1.1 {
            if v2.0.is_some() {
                v.push(v2.0.unwrap());
            }
            while let Some(ln) = list2 {
                v.push(ln.val);
                list2 = ln.next;
            }
            break;
        }

        if v2.1 {
            if v1.0.is_some() {
                v.push(v1.0.unwrap());
            }
            while let Some(ln) = list1 {
                v.push(ln.val);
                list1 = ln.next;
            }
            break;
        }
    }

    let mut result = None;
    for i in v.into_iter().rev() {
        result = Some(Box::new(ListNode {
            val: i,
            next: result,
        }));
    }
    result
}
