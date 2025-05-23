#[derive(Debug)] // Class 168 e 169

enum PaymentMethodType {
    Credicard(String), 
    Debitcard(String),
    PayPal(String, String),
}


fn main() {
    let mut my_payment_Method = PaymentMethodType::Credicard(String::from("0034 5678 9012 3456"));

    my_payment_Method = 
    PaymentMethodType::PayPal(String::from("bob@email.com"), String::from("password"));

    println!("{:?}", my_payment_Method);
}

/*
#[derive(Debug)]

enum Cardsuit { // Class 167
    Spades,
    Hearts,
    Diamonds,
    Clubs,
    }
    struct Card {
    rank: String,
    suit: Cardsuit,
    }
 fn main
    let first_card = Cardsuit::Hearts;
    let mut second_card = Cardsuit::Spades;
    second_card = Cardsuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [Cardsuit::Spades, Cardsuit::Hearts];
    let card_suits = [Cardsuit::Spades, Cardsuit::Diamonds];*/