use std::collections::HashMap;
fn main() {
 let vec = vec![1,2,3,4,5,6,7,8,9,10];
 // todo: Create an iterator from the vector above.
 // todo: Using filter() to filter for numbers > 5, and use map() to multiply their values by 2.
 // todo: Save the values into a new vector called vec_even
 // todo: print vec_even


// .iter() returns a &
// .into_inter() takes ownership of the values in vector
let mut iterator = vec.iter();
 
// iter returns a reference type variable, filter returns a reference type variable, therefore the chaining of both leads to a double reference value
//  let vec_even: Vec<i32> = iterator.filter(|&&x| x> 5)
//  let vec_even: Vec<i32> = iterator.filter(|x| x> &&5)
let vec_even: Vec<i32> = iterator.filter(|x| **x> 5)
    .map(|num| num *2).collect();

println!("{:?}", vec_even);

let mut stock_prices = HashMap::new();
stock_prices.insert("TSLA", 20);
// todo: insert a few more stock_prices, less or more than 50 dollars
// todo: use a for loop to print out the name of each stock that costs less than 50 dollars
stock_prices.insert("JPM", 51);
stock_prices.insert("BLK", 49);
stock_prices.insert("AAPL", 100);
stock_prices.insert("GOOGL", 30);
 
for (ticker, price) in stock_prices.iter().filter(|x| *x.1< 50){
   // both works: if *price < 50
   println!("{}", ticker);

   // if *v < 50 {
   //    println!("{}", k);
   // }

    

}



}
