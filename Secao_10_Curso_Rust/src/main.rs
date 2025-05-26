enum Milk {
    Lowfat(u32),
    Whole,
    NonDairy { Kind: String }, 
}

fn main() {// Class 180
    let my_beverage = Milk::NonDairy { 
        Kind: String::from("Oat"), 
    }; 

    let Milk::NonDairy { Kind } = my_beverage else {
        println!("You do not have the nondairy milk.");
 return;
    };
        println!("{} milk is available here.", Kind);
    }
    
/*
enum Milk {
    Lowfat(u32),
    Whole,
    NonDairy { Kind: String }, // Class 179
}

fn main() {
    let my_beverage = Milk::Whole; // Class 179

    if let Milk::NonDairy { Kind } = my_beverage {
        println!("You beveraged is {} milk.", Kind);
    } else {
        println!("You have a regular milk beverage.");
    }
        
}
*/
/*
enum Milk {
    Lowfat(u32), // Class 178
    Whole
}

impl Milk {
    fn drink(self) {
        match self { //Granular match
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk! is my favorite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got lowfat {}% version.", percent);
            }
            Milk::Whole => {
                println!("Whole milk is great for cooking!");
            }
        }
    }
}

fn main() {
    Milk::Lowfat(1).drink(); // Class 178
    Milk::Lowfat(2).drink();
    Milk::Whole.drink();
}
*/
/*
#[derive(Debug)] // Class 177

enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus { // Class 177
    fn check(&self) {
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Your order has been delivered!");
            }
            other_status => {
                println!("Your order is currently: {:?}", other_status);
            }
        }
    }
}

fn main() { //Catching multiple enum variants
    OnlineOrderStatus::Ordered.check();
}
*/

/*
enum LaundryCycle {
    Cold,
    Hot {temperature: u32},
    Delicate(String),
}
// Implementation block for LaundryCycle enum
impl LaundryCycle { // Class 176
    fn wash_laundry(&self) {
    match self {
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

}

fn main() {
    LaundryCycle::Cold.wash_laundry();
    // Using the enum variant with a value and calling the method
    let hot_cycle = LaundryCycle::Hot { temperature: 100 };
    hot_cycle.wash_laundry();

    let delicate_cycle = LaundryCycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundry();
}


*/

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
