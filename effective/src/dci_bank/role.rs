
use crate::dci_bank::data::{Account, AccountTransaction, AccountOperation};

use super::data::BankOperation;

/// Account role for making money deposit
pub trait CheckingAccount {
    fn deposit(&mut self, amount: f64) -> f64;
}

impl CheckingAccount for Account {
    fn deposit(&mut self, amount: f64) -> f64 {
        self.balance += amount;
        self.balance()
    }
}

/// Account role for making money withdrawal
pub trait SavingsAccount {
    fn withdraw(&mut self, amount: f64) -> f64;
}

impl SavingsAccount for Account {
    fn withdraw(&mut self, amount: f64) -> f64 {
        self.balance -= amount;
        self.balance()
    }
}

/// Account role for locking the account for operations
pub trait SynchronizedAccount {
    fn lock(&mut self) -> bool;
    fn unlock(&mut self) -> bool;
}

impl SynchronizedAccount for Account {
    fn lock(&mut self) -> bool {
        if !self.locked() {
            self.locked = true;
            true
        } else {
            false
        }
    }

    fn unlock(&mut self) -> bool {
        if self.locked() {
            self.locked = false;
            true
        } else {
            false
        }
    }
}

/// Account operations log
pub trait AccountOperationLogger {
    fn log(operation: & AccountOperation, transaction: &AccountTransaction);
}

pub trait BankOperationLogger {
    fn log(operation: &BankOperation, transaction: &AccountTransaction);
}



