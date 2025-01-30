fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(x: &mut String) {
    x.push_str("Philidelphia");
}

fn visit_new_york(x: &mut String) {
    x.push_str("New York");
}

fn visit_boston(x: &mut String) {
    x.push_str("Boston");
}

fn show_itinerary(x: &String) {
    println!("{x}")
}

fn main() {
    let mut trip: String = start_trip();

    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push('.');
    show_itinerary(&trip);
}