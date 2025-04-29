fn main() {
    //Class 74
    let result = square(5);
    println!(" The square of 5 is {result}");

}
    //Function declaration outside of main
    fn square(number: i32) -> i32 {
     return number * number;
    }





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
