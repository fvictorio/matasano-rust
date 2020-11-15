use matasano::hex_to_base64;

fn main() {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let result = hex_to_base64(s);

    println!("{}", result);
}
