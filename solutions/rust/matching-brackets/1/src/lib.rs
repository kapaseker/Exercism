pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];
    for i in string.chars() {
        match i {
         '[' | '(' | '{' => brackets.push(i),
         ']' =>{
            if !matches!(brackets.last(), Some('[')) {
                return false;    
            }
            brackets.pop();
         },
         ')' =>{ 
            if !matches!(brackets.last(), Some('(')) {
                return false;   
            }
            brackets.pop();
         },
         '}' => { 
            if !matches!(brackets.last(), Some('{')) {
                return false;    
            }
            brackets.pop();
         },
         _ => {},
        }
    }  
    brackets.is_empty()
}
