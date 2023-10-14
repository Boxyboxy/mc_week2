// define struct of UserAccount with field: name (String), and age (Option<u32>)
struct UserAccount {
    name:String, 
    age: Option<u32>
}
// define a trait called Balance, and within, function get_balance returning integer of 10
trait Balance {
    fn get_balance() -> u32{
        10
    }

    fn increase_balance(&mut self, amount:u32) -> Result<u32, String>;
}
// implement trait Balance to UserAccount struct
// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
// within this function,
// - if increase amount is <= 10, return an OK containing the get_balance + amount
// - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
// Tip: this function should return a Result<u32, String>
impl Balance for UserAccount{
    fn increase_balance(&mut self, amount:u32) -> Result<u32, String>{
        if amount <= 10 {
            Ok(<UserAccount as Balance>::get_balance() + amount)
        } else {
            Err("Increase must be less than 10!".to_owned())
        }
    }
}

fn main() {
    // create user_account, and set his age as Option::None
    let mut user_account = UserAccount {
        name: "John".to_owned(),
        age: Option::None
    };
    // You want to increase the user_account's balance by 11
    // use a match, if the result of increase_balance is
    // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
    // - Err: print the error message returned

    match user_account.increase_balance(11) {
        Ok(x) => println!("{}'s balance increased to {}",user_account.name, x),
        Err(err)=> println!("{}", err)
    }

    // use an if...let...else statement to print the UserAccount age if it is a Option::Some
    if let Some(x) = user_account.age {
        println!("{}'s age is {}",user_account.name ,x);   
    }else {
        println!("Age is not present for {}", user_account.name);
    }


    // create user_account, and set his age as Option::None
    let mut user_account_2 = UserAccount {
        name: "Mary".to_owned(),
        age: Option::Some(5)
    };
    // You want to increase the user_account_2's balance by 11
    // use a match, if the result of increase_balance is
    // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
    // - Err: print the error message returned

    match user_account_2.increase_balance(9) {
        Ok(x) => println!("{}'s balance increased to {}", user_account_2.name, x),
        Err(err)=> println!("{}", err)
    }
    
    // use an if...let...else statement to print the UserAccount age if it is a Option::Some
    if let Some(x) = user_account_2.age {
        println!("{}'s age is {}",user_account_2.name , x);   
    }else {
        println!("Age is not present for {}", user_account_2.name);
    }
    
}