fn main() {
    println!("\t{title1:<15}{title2:<10}", title1="Item", title2="Projected");
    println!("\t=============  ==========");
    println!("\t{item:<15}${amount:>9}", item="Income", amount=format!("{:.2}", 1000.00));
    println!("\t{item:<15}${amount:>9}", item="Taxes", amount=format!("{:.2}", 100.00));
    println!("\t{item:<15}${amount:>9}", item="Tithing", amount=format!("{:.2}", 100.00));
    println!("\t{item:<15}${amount:>9}", item="Living", amount=format!("{:.2}", 650.00));
    println!("\t{item:<15}${amount:>9}", item="Other", amount=format!("{:.2}", 90.00));
    println!("\t=============  ==========");
    println!("\t{item:<15}${amount:>9}", item="Delta", amount=format!("{:.2}", 60.00));
}
