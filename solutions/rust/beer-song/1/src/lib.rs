pub fn verse(n: u32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if n == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else if n == 2 {
        "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string()
    } else {
        format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {next} bottles of beer on the wall.\n", n = n, next = n - 1).to_string()
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut str_list: Vec<String> = vec![];
    let mut i = start;
    loop {
        str_list.push(verse(i));
        if i > end {
            i -= 1;
        } else {
            break;
        }
    }
    str_list.join("\n")
}
