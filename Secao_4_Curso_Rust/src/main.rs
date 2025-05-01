/* fn apply_to_jobs(number: i32, title: &str) {
    println!("Applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("b"))
}
*/
/*
//Class 85
fn even_odd(number: i32) {
    
    // Semelhante a um if ternário
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");
}*/
fn main() {
let evaluation = false;

let value = match evaluation {
    true => 20,
    false => 40,
};

println!("The value is {value}");

}
/*
    //Class 85
    even_odd(55);
    even_odd(100);
*/

/*
    //Class 84
    let season = "spring";
    if season == "summer" {
        println!("Scools out!");
    } else if season == "winter" {
        println!("Brr it's cold!");
    } else {
        println!("Lots of rain!");
    }
 /*
 //Class 83
 let season = "summer";
 if season == "summer" {
     println!("Scools out!");
 } else if season == "winter" {
     println!("Brrr, it's cold!");
 } else if season == "fall" {
     println!("Leaves are falling!");
 } else if season == "spring" {
     println!("Lots of rain!");
 } else {
     println!("I don't know that season");
 }
}
*/
    // Class 78
    apply_to_jobs(35, "Rust Developer");

    println!("{}", is_even(4));
    println!("{}", is_even(5));

    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));
*/

/*
    //Class 77
    let multiplier = 3;

    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };

    println!("The result of the calculation is {calculation}");
*/
/*
       //Class 76
    let result = mystery();

}
    //Function declaration outside of main
    fn mystery() {
        println!("Hellou there");
    }
     */

/*
  //Class 74
    let result = square(5);
    println!(" The square of 5 is {result}");
    let result = square(13);
    println!(" The square of 13 is {result}");



}
    //Function declaration outside of main
    fn square(number: i32) -> i32 {
     return number * number;
    }

*/

/*
    // Aula 72 e 73
    open_store("Brookling");
    bake_pizza(20, "pepperoni");
    swing_in_profit();
    swing_in_profit();
    open_store("Queens");
    bake_pizza(15, "Mussarela");

}

fn open_store(neighborhood: &str) {
    let _ = neighborhood;
    println!("Opening my pizza store");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas");
}

fn swing_in_profit() {
    println!("So much $$$, so little time");
}
*/
