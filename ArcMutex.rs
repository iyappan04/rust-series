use std::sync::{Arc, Mutex};
use std::thread;


// Arc allows data to be shared accross multiple threads, while mutex ensure's that only one thread can modify the data.

#[derive(Debug)]
struct BankAccount {
    id: u32,
    balance: f64,
}

impl BankAccount {

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds for withdrawal from account {}", self.id);
        }
    }
}

fn main() {
    
    let account1 = Arc::new(Mutex::new(BankAccount { id: 1, balance: 100.0 }));
    let account2 = Arc::new(Mutex::new(BankAccount { id: 2, balance: 200.0 }));

    
    let mut handles = vec![];

    for i in 0..5 {
        let account1_clone = Arc::clone(&account1);
        let account2_clone = Arc::clone(&account2);

        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                let mut account = account1_clone.lock().unwrap();
                account.deposit(50.0);
                println!("Thread {} deposited $50 into account {}", i, account.id);
            } else {
                let mut account = account2_clone.lock().unwrap();
                account.withdraw(100.0);
                println!("Thread {} withdrew $100 from account {}", i, account.id);
            }
        });

        handles.push(handle);
    }

    
    for handle in handles {
        handle.join().unwrap();
    }


    println!("Final balance for account 1: {:?}", account1.lock().unwrap());
    println!("Final balance for account 2: {:?}", account2.lock().unwrap());
}
