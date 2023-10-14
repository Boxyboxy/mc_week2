// create a function is_this_cash which takes in string and returns Result of String
 // if input of string equals to "cash", return Ok of "Yes this is cash" -> Ok(“yes this is cash”)
 // else return Err of "No, this is not cash" -> Err(“no, this is not cash”)
fn is_this_cash(input: String) -> Result<String, String> {
    if input == "cash".to_owned() {
        Ok("yes this is cash".to_owned())
    } else {
        Err("no, this is not cash".to_owned())
    }
}
fn main() {
// create variable cash of type Option<String>, assign "cash"
let cash = Some("cash".to_owned());

// create variable credit of type Option<String>, assign "credit"
let credit = Some("credit".to_owned());
// using if let on cash, bind variable within Some -> if let Some(x) = var_name {}
// pass the binded variable into is_this_cash function and use match
// if matches on OK, print "Output X", where X the value within OK
// if matches on Err, print "Output X", where X the value within Err
if let Some(input) = cash {
    match is_this_cash(input) {
        Ok(x) => println!("Output: {}", x),
        Err(error) => println!("Output: {}", error)
    }
}

 // do the same as above on credit
if let Some(input) = credit {
    match is_this_cash(input) {
        Ok(x) => println!("Output {}", x),
        Err(error) => println!("Output {}", error)
    }
}
}