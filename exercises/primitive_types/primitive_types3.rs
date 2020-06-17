// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is. 
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    let arr: [u8; 1] = [1];

    if arr.len() >= 100
    {
        println!("Wow, that's a big array!");
    }
    else
    {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
