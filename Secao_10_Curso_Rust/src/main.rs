#[derive(Debug)] // Class 172
enum Beans {
    Pinto,
    Black,
}
// It is important to derive Debug for the enum
#[derive(Debug)] 
enum Meat {
    Chicken,
    Steak,
}
#[derive(Debug)]
enum RestaurantItem {
    Burrito {meat: Meat, beans: Beans},
    Bowl {meat: Meat, beans: Beans },

    VeganPlate, 
}

fn main() { // Class 172
    let lunch = RestaurantItem::Burrito {
        meat: Meat::Steak, 
        beans: Beans::Pinto};

    let dinner = RestaurantItem::Bowl{meat: Meat::Chicken, beans: Beans::Black};
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!("Lunch was {:?} and dinner was {:?}", lunch, dinner);
    println!("Nobody ate the vegan plate");
}
/*Class 171
#[derive(Debug)] // It is important to derive Debug for the enum
enum PaymentMethodType {
    Credicard(String), 
    Debitcard(String),
    PayPal { username: String, password: String }, 
    Cash,
}
fn main() {
    let visa = PaymentMethodType::Credicard(String::from("1234-5678-9012-3456"));
    let paypal = PaymentMethodType::PayPal {
        username: String::from("bob@gmail.com"),
        password: String::from("password123"),
    };
    println!("{:?}", paypal);
}
*/
/*
#[derive(Debug)] // Class 168 e 169

struct Credentials {
    username: String,
    password: String,
}
#[derive(Debug)] // It is important to derive Debug for the enum
enum PaymentMethodType {
    Credicard(String), 
    Debitcard(String),
    PayPal(Credentials),
}
fn main() {
    let paypal_credentials = Credentials {
        username: String::from("bob@gmail.com"),
        password: String::from("password123"),
    };

    let paypal = PaymentMethodType::PayPal(paypal_credentials);
    println!("{:?}", paypal);
}
*/
/*
enum PaymentMethodType { // outside of main
    Credicard(String), 
    Debitcard(String),
    PayPal(String, String),
}
    //fn main(){
        let mut my_payment_Method = PaymentMethodType::Credicard(String::from("0034 5678 9012 3456"));

    my_payment_Method = 
    PaymentMethodType::PayPal(String::from("bob@email.com"), String::from("password"));

    println!("{:?}", my_payment_Method);
}
*/
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