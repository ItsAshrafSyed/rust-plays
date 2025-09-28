// structs are used to create custom data types that group related values together
pub struct BankAccount {
    pub owner: String,
    pub balance: f32,
}
impl BankAccount {
    println!("--- Struct and Impl ---");
    pub fn withdraw(&mut self, amount: f32) {
        self.balance -= amount;
        println!("Withdrew: {}", amount);
    }

    pub fn check_balance(&self) {
        println!("Owner: {}, Balance: {}", self.owner, self.balance);
    }
}
