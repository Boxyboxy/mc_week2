// create get_first_element<T> function taking in (vec: Vec<T>) and outputs a Option<T> type
 // combine and use into_iter() and next() to obtain first element

fn get_first_element<T>(vec: Vec<T>) -> Option<T> {
  vec.into_iter().next()
}


fn main() {
// create vec1 with 3 number items
// create vec2 with 0 item
// obtain first_element1 by passing in vec1 into get_first_element
// obtain first_element2 by passing in vec2 into get_first_element
// match on first_element1
// if Some, print "The first element is <first item>"
// if None, print "The vector is empty!"
// match on first_element2
// if Some, print "The first element is <first item>"
// if None,

let  v1: Vec<i32> = vec![1, 2, 3]; 
let  v2: Vec<_> = Vec::new();
let first_element1: Option<i32> = get_first_element(v1);
let first_element2: Option<i32> = get_first_element(v2);
match first_element1{
  Some(x) => println!("{}", x),
  None => println!("The vector is empty!")
} 

match first_element2 {
  Some(x) => println!("{}", x),
  None => println!("The vector is empty!")
} 
}