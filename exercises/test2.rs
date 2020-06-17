// test2.rs
// This is a test for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `Strings`, some are `&strs`. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!


fn string_slice(arg: &str)
{
    println!("{}", arg);
}
fn string(arg: String)
{
    println!("{}", arg);
    if true
    {
        println!("some other shit")
    }
}
fn main()
{
    // The ones I got wrong are marked with an X
    string_slice("blue"); // X
    string("red".to_string());
    string(String::from("hi"));
    // What is to_owned() ??
    string("rust is fun!".to_owned()); // X
    // What is .into() ??
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // X
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // X
}
