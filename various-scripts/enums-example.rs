
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("hi");
    let f1 = value_in_cents(Coin::Dime);
    println!("Value in cents {}", f1)
}

fn value_in_cents(coin: Coin) -> u8 {
    //let coin = Coin::Penny;
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


//enum Color {
//    Red,
//    Green,
//    Blue,
//}

//fn colorf() {
//    let color = Color::Blue;
//    match color {
//        Color::Red => println!("The color is red!"),
//        Color::Green => println!("The color is green!"),
//        Color::Blue => println!("The color is blue!"),
//    }
//}
