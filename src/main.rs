fn main() {
  // work with structs
let mut account = BankAccount{
    owner : "Alice".to_string(),
    balance : 1200.45,
};
 
 // immutable account check balance
account.check_balance();

// mutable borrow to withdraw money
account.withdraw(230.1);

// check balance again
account.check_balance();

}

struct BankAccount {
    owner : String,
    balance : f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by  {} has a balance of {}", self.owner, self.balance);
    }
}