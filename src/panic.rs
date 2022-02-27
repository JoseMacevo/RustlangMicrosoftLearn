pub fn run() {
    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!!"),
        _ => {}
    }
}
