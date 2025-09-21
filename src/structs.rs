pub fn struct_and_impl() {
    println!("--- Struct and Impl ---");
}
pub struct BankAccount {
    pub owner: String,
    pub balance: f32,
}
impl BankAccount {
    pub fn withdraw(&mut self, amount: f32) {
        self.balance -= amount;
        println!("Withdrew: {}", amount);
    }

    pub fn check_balance(&self) {
        println!("Owner: {}, Balance: {}", self.owner, self.balance);
    }
}
