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
fn main() {
    let first_card = Cardsuit::Hearts;
    let mut second_card = Cardsuit::Spades;
    second_card = Cardsuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [Cardsuit::Spades, Cardsuit::Hearts];
    let card_suits = [Cardsuit::Spades, Cardsuit::Diamonds];
}
