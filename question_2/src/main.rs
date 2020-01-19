mod lib;

fn main(){
    let cust_name= String::from("Muneeb");
    lib::services::plumbing_services::new_order(&cust_name);
}