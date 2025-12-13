use std::io;
fn main() {
    println!("enter any text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let input = input.trim();
    let input_reversed:Vec<char> = reverse_string(&input);
    println!("Original input was: {:?}", input);
    println!("your text is {:?}", input_reversed.iter().collect::<String>());

}

fn reverse_string(text: &str) -> Vec<char>{
    let input_len = text.chars().count();
    let mut v = vec![' ';input_len];
    for (index, char) in text.chars().enumerate() {
        v[input_len-index-1] = char;
    }
    v
}