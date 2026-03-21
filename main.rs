// fn main (){
//     let s1 = String::from("Rust");
//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }

// fn main(){
//     let s1 = String::from("Hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);

// }

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }



fn main (){
    // let mut _x = 5;
    // let _r = &mut _x;
    // *_r += 1;
    // *_r -= 3;

    // println!("The value of _x is: {}", _x);
    // println!("The value of _r is: {}", _r);


    let mut account = BankAccount{
        owner : "Alice".to_string(),
        balance : 150.55,
    };

    account.check_balance();
    account.withdraw(45.25);
    account.check_balance();


}

struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount : f64){
        println!("Withdrawing ${} from the account, owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has The current balance of ${}", self.owner, self.balance);
    }
}