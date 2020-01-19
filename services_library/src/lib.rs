pub mod services{
    pub mod painting_services{
        pub fn new_order(cust_name:&String){
            println!("{}  has placed a new order for Paint work.", cust_name);
        }
    }
}