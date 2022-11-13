fn main() {
    println!("The escape sequences are:");
    println!("\t{name:<9}\\n", name="Newline");
    println!("\t{name:<9}\\t", name="Tab");
    println!("\t{name:<9}\\\\", name="Slash");
    println!("\t{name:<9}\\'", name="SQuote");
    println!("\t{name:<9}\\\"", name="DQuote");
}
