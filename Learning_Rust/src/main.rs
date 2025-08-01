
}

/*
----------------------------------------------------------------
*/


// fn print_gt_number(num1: &u8, num2: &u8){
//     println!("The given number is {}, which is grater than {}", num1, num2);
// }

// fn print_lt_number(num1: &u8, num2: &u8){
//     println!("The given number is {}, which is lesser than {}", num1, num2);
// }


// fn main() {
//     let num1: u8 = 10;
//     let num2: u8 = 20;

//     match num1>num2 {
//         true => print_gt_number(&num1, &num2),
//         false => print_lt_number(&num1, &num2),
//     }
    
// }





/*
----------------------------------------------------------------
*/


// enum Store{
//     Apple,
//     Orange,
//     Pineapple,
// }

// struct Customer{
//     name: String,
//     age: u8,
//     purchases: Vec<Store>,
// }

// fn print_customer(customer: &Customer) {
//     println!("Customer Name: {:?}, Age: {:?}", customer.name, customer.age);
//     println!("Purchases: {:?}", customer.purchases);
// }

// #[warn(dead_code)]
// fn main() {
//     let customer1: Customer = Customer{
//         name:"Karthik".to_string(),
//         age: 23,
//         purchases: vec![Store::Apple, Store::Orange, Store::Pineapple],
//     };
//     print_customer(&customer1);
// }


/*
----------------------------------------------------------------
*/


// mod day_1;  // Import the module

// use day_1::sample_boolean;  // Use the function
// use day_1::varibles; // Variables

// fn main() {
//     println!("Hello, world!");
    
//     // Im theis we are doing a seperate functional calls
//     sample_boolean(10);
//     varibles();  // Call the function
    
//     // here is how we are importing it from mod day_1 
//     day_1::string_formating("Karthik".to_string(),"Shoping".to_string(),"Abheram".to_string(),  23);  // Call the function again to test it again.
//     day_1::mathematical();

//     let mut b = String::from("Hello");
//     b.push_str(", World");
//     println!("{}, {}",b, b.len());
// }
