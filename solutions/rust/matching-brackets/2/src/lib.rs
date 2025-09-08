pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];
    for i in string.chars() {
        match i {
         '[' | '(' | '{' => brackets.push(i),
         ']' =>{
            if !matches!(brackets.pop(), Some('[')) {
                return false;    
            }
         },
         ')' =>{ 
            if !matches!(brackets.pop(), Some('(')) {
                return false;   
            }
         },
         '}' => { 
            if !matches!(brackets.pop(), Some('{')) {
                return false;    
            }
         },
         _ => {},
        }
    }  
    brackets.is_empty()
}
