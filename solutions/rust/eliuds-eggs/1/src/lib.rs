pub fn egg_count(display_value: u32) -> usize {
    let mut eggs:usize = 0;
    let mut display_value = display_value;
    while display_value != 0 {
        if display_value & 1 == 1 {
           eggs += 1; 
        }
        display_value = display_value >> 1;
    }
    eggs
}
