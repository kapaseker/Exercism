#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list { return Comparison::Equal; }

    if _first_list.is_empty() { return Comparison::Sublist; }
    if _second_list.is_empty() { return Comparison::Superlist; }

    let a_len = _first_list.len();
    let b_len = _second_list.len();

    if a_len < b_len {
        let len = b_len - a_len;
        for i in 0..=len {
            if _second_list[i..i + a_len] == _first_list[..] {
                return Comparison::Sublist;
            }
        }
    } else if a_len > b_len {
        let len = a_len - b_len;
        for i in 0..=len {
            if _first_list[i..i + b_len] == _second_list[..] {
                return Comparison::Superlist;
            }
        }
    }


    return Comparison::Unequal;
}
