fn main() {
    //Class 128 Project
    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push_str(".");
    show_itinerary(&trip);
}
fn start_trip() -> String {
    String::from("The plan is...")
}
fn visit_philadelphia(trip: &mut String) {
    trip.push_str("Philadelphia");
}
fn visit_new_york(trip: &mut String) {
    trip.push_str("New York");
}
fn visit_boston(trip: &mut String) {
    trip.push_str("boston");
}
fn show_itinerary(trip: &String) {
    println!("{}", trip);
}

/*
    // Class 126
    let registration = (true, false, true);
    let first = registration.0;
    println!("First: {} and {:?} ", first, registration);

    let languages = (String::from("Rust"), String::from("Python"));
    let first = &languages.0;
    println!("First: {} and {:?} ", first, languages);
*/
/*
 //Class 125
    let city = create_city();
    println!("{}", city);


//Class 125 outside main
fn create_city() -> String {
    String::from("New York")
}
*/

/*
  //Class 124
    let mut coffee = String::from("mocha");
    let a = &mut coffee;
    println!("{}", a);
    let b = a;
    println!("{}", b);
*/
/*
    // Class 122
    let car = String::from("red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{} and {} and {}.", ref1, ref2, &car);
*/
/*
//class 121
let mut current_meal = String::new();

add_flour(&mut current_meal);
show_my_meal(&current_meal);
*/
//Class 121 functions outside main
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
