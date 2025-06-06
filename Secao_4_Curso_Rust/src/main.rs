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
/*
fn countdown(seconds: i32) { //Class 92
    if seconds == 0 {
        println!("Blast off!");
    } else {
        println!("{seconds} seconds to blast off!");
        countdown(seconds - 1); // Chamada recursiva
    }
}
*/
/* //Class 97 Project 
fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}*/
/* //Class 97 Project
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}*/
/* Class 97 Project with for loop
fn factorial_iterative(n: i32) -> i32 {
    let mut product = 1;
    for i in 1..=n {
        product *= i;
    }
    product
} */
/* Class 97 Project with while loop
fn factorial_iterative(n: i32) -> i32 {
    let mut product = 1;
    let mut count = n;

    while count > 0 {
        product *= count;
        count -= 1;
    }
    product
}*/
fn factorial_recursive(n: i32) -> i32 {
    if n == 1 {
    return 1;   
}
    n * factorial_recursive(n - 1)
}
fn main() {
    /*Class 97 Project
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number("green"));
    println!("{}", color_to_number("blue"));
    println!("{}", color_to_number("purple"));
    println!("-----------------------");
*/
    //println!("{}", factorial_iterative(5));
    println!("{}", factorial_recursive(5));
}
/*
    //Class 92
    countdown(5);
    countdown(5);
*/
/*
    //Class 91
    let mut seconds = 21;

    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipiing 3 seconds...");
            seconds -= 3;
            continue;
        }
        println!("{seconds} seconds to blast off!");
        seconds -= 1;
    }
    println!("Blast off!");
*/
/*
    //Class 90
    let mut seconds = 21;

    loop {
        if seconds <= 0 {
            println!("Blast off!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipiing 3 seconds...");
            seconds -= 3;
            continue;
        }
        println!("{seconds} seconds to blast off!");
        seconds -= 1;
    }
*/
/*    //Class 89
    let mut seconds = 10;

    loop {
        if seconds == 0 {
            println!("Blast off!");
            break;
        }
        println!("{seconds} to blast off!...");
        seconds -= 1;
    }
*/
/*
    //Class 88
    let number = 8;

    match number {
        /*
         2 | 4| 6| 8 => println!("The number is even"),
         1 | 3| 5| 7 => println!("The number is odd"),
         _ => println!("Unknow for now"),
        */
        value if value % 2 == 0 => println!("The number is even"),
        x if x % 2 != 0 => println!("The number is odd"),
        _ => unreachable!(), // unreachable!() é uma macro que indica que o código nunca deve chegar a esse ponto
    }

*/
/*
    //Class 87
    let season = "summer";

    match season {
        "summer" => println!("Scools out!"),
        "winter" => println!("Brr it's cold!"),
        //Undercore (_) é o mesmo que else, cobrindo todos os outros casos
        _ => println!("I don't know that season"),
    }
*/
/*
    //Class 86
    let evaluation = false;

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("The value is {value}");
*/
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
