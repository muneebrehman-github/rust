pub mod services{
    pub mod plumbing_services{
        pub fn new_order(cust_name:&String){
            println!("{}  has placed a new order for Plumbering work.", cust_name);
        }
    }
}