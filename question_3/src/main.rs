use services_library;

fn main() {
    let cust_name= String::from("Muneeb");
    services_library::services::painting_services::new_order(&cust_name);

}
