mod services{
    pub mod electricity_services{
        pub fn new_order(cust_name:&String){
            println!("{}  has placed a new order for Electric work.", cust_name);
        }
    }
}

fn main(){
    let cust_name= String::from("Muneeb");
    services::electricity_services::new_order(&cust_name);
}