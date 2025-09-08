pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions = vec![];
    if (n & 1) == 1 {
        actions.push("wink");
    }
    
    if (n & 2) == 2 {
        actions.push("double blink");
    }
    
    if (n & 4) == 4 {
        actions.push("close your eyes");
    }
    
    if (n & 8) == 8 {
        actions.push("jump");
    }
    
    if (n & 16) == 16 {
        actions.reverse();
    }
    
    return actions;
}
