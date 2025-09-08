pub fn number(user_number: &str) -> Option<String> {
    let mut phone: Vec<u8> = vec![];

    for x in user_number.chars() {
        if x.is_ascii_digit() {
            phone.push(x.to_digit(10)? as u8)
        }
    }

    if phone.len() == 11 {
        if phone[0] != 1 {
            return None;
        }
        phone.remove(0);
    }

    if phone.len() == 10 {
        if phone[0] > 1 && phone[3] > 1 {
            Some(phone.iter().map(|c| char::from(b'0' + *c)).collect())
        } else {
            None
        }
    } else {
        None
    }
}
