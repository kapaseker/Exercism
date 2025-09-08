/// What should the type of _function be?
pub fn map<T, F, O>(input: Vec<T>, mut x: F) -> Vec<O>
where
    F: FnMut(T) -> O,
{
    let mut vec = Vec::with_capacity(input.len());
    for i in input {
        vec.push(x(i));
    }
    vec
}
