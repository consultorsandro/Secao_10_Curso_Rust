fn main() {
    //122
    let car = String::from("Red");
    let ref1 = &car;  
    let ref2 = &car;
    println!("{} and {} and {}", ref1, ref2, &car);
}
/*
//class 121
let mut current_meal = String::new();

add_flour(&mut current_meal);
show_my_meal(&current_meal);
*/


//meal: String
//mut meal : String
//meal: &String
//meal: &mut String
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {}", meal);
}