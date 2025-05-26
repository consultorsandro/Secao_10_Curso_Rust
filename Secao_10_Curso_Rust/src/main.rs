enum LaundryCycle { // Class 175
    Cold,
    Hot {temperature: u32}, 
    Delicate(String),
}
fn main() { 
    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 100 });
    wash_laundry(LaundryCycle::Delicate(String::from("Silk")));
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Runing the laundry with cold temperature");
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running the laundry with hot temperature: {}°C", temperature);
        }
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running the laundry with delicate cycle for: {}", fabric_type);
        }
    }
}


/*
enum OperatingSystem { // Class 173
    Windows,
    MacOS,
    Linux,
}
fn main() { // Class 173
    let my_computer = OperatingSystem::Linux;
    let age = years_since_release(my_computer);
    println!("My computer is {} years old", age);

    let dads_computer = OperatingSystem::Windows; // Class 174
    let age = years_since_release(dads_computer);
    println!("My dad's computer is {} years old", age);  
}
fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system");
            39

        }
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 31,
    }
*/

/*
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

*/
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