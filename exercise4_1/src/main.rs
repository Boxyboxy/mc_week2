mod transaction;

// use transaction::withdrawal;
// use transaction::deposit;
// use transaction::transfer;
use transaction::*;

enum TransactionType {
 Withdrawal,
 Deposit,
 Transfer,
}
fn main() {
    let transaction = TransactionType::Withdrawal;
    match transaction {
        TransactionType::Withdrawal => withdrawal(),
        TransactionType::Deposit => deposit(),
        TransactionType::Transfer => transfer(),
    } 
    let transaction1 = TransactionType::Deposit;
    match transaction1 {
        TransactionType::Withdrawal => withdrawal(),
        TransactionType::Deposit => deposit(),
        TransactionType::Transfer => transfer(),
    }
}