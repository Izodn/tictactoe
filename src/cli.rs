/// Print/Output
///
/// # Arguments
///
/// * msg: The string to output
pub fn print(msg: String) {
    println!("{}", msg);
}

/// Display some text, and return input
///
/// # Arguments
///
/// * msg: The string to output
pub fn prompt(msg: String) -> String {
    print(msg);
    input()
}

/// Capture user input to string
///
/// Returns a string that is the user input
pub fn input() -> String {
    use std::io;

    //Create a string that will hold the user input
    let mut input_str = String::new();

    //Assign read_line() to nthing, so the compiler doesn't complain
    let _ = io::stdin().read_line(&mut input_str);

    //Return trimmed input
    input_str.trim().to_string()
}
