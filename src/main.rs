struct Bank {
    users: Vec<User>,
    safe: u128,
    fee: u8,
    transactions: Vec<Transaction>,
}

impl Bank {
    pub fn new(safe: u128, fee: u8) -> Bank {
        Bank {
            users: Vec::new(),
            safe,
            fee,
            transactions: Vec::new()
        }
    }

    pub fn new_user(self: &mut Bank, user: User) {
        self.users.push(user);
    }

    pub fn new_transaction(self: &mut Bank, tran: Transaction) {
        self.transactions.push(tran);
    }

    pub fn update_fee(self: &mut Bank, fee: u8) {
        self.fee = fee;
    }
}

struct User {
    name: String,
    account_id: String,
    balance: u128,
}

impl User {
    pub fn new(name: String, account_id: String, balance: u128) -> Self {
        Self {
            name,
            account_id,
            balance,
        }
    }
    pub fn deposit(&mut self, amount: u32) {
        self.balance = self.balance + amount as u128;
    }

    pub fn withdraw(&mut self, amount: u32) {
        self.balance -= amount as u128;
    }

    pub fn transfer(bank: &mut Bank, sender: &mut User, receiver: &mut User, amount: u32) {
        // calculate the protocol fee 
        let fee = ((amount as u128) * (bank.fee as u128) / 100) as u32;        
        // check the total balance + fee should be greater than amount
        assert!(sender.balance + fee as u128 >= amount as u128, "Insufficent balance");
        // Decrease the sender balance 
        sender.balance = sender.balance - (amount + fee as u32) as u128;
        // increase the receiver balance 
        receiver.balance = receiver.balance + amount as u128;
        // update the bank safe with the fee
        bank.safe += fee as u128;
    }
}

struct Transaction {
    sender: String,
    receiver: String,
    amount: u32,
}

// impl Transaction {

// }

struct PaymentProcessor {
    banks: Vec<Bank>,
}

impl PaymentProcessor {
    pub fn new () -> Self {
        PaymentProcessor {
            banks: Vec::new()
        }
    }

    pub fn add(&mut self, bank: Bank) {
        self.banks.push(bank);
    }
}


fn main() {
        // Create a bank
        let mut bank = Bank::new(0, 5);

        let mut user1 = User::new(String::from("A1"), String::from("A1"), 0);
        let mut user2 = User::new(String::from("A1"), String::from("A1"), 0);
        
        User::deposit(&mut user1, 1000);
        User::transfer(&mut bank, &mut user1, &mut user2, 800);

        Bank::new_user(&mut bank, user1);
        Bank::new_user(&mut bank, user2);

        let mut proces = PaymentProcessor::new();
        PaymentProcessor::add(&mut proces, bank);

        for user in &proces.banks[0].users {
            println!("{}'s balance: {}", user.name, user.balance);
        }
}
