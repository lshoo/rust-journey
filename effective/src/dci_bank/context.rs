
extern crate pretty_env_logger;

use std::collections::HashMap;
use std::collections::VecDeque;

use crate::dci_bank::data::{Account, AccountTransaction, AccountTransactionError};
use crate::dci_bank::role::{SavingsAccount, CheckingAccount, SynchronizedAccount, AccountOperationLogger, BankOperationLogger};

use super::data::{AccountOperation, BankOperation};


/// Represents a list of account, not actually part of the business domain
pub type AccountMap = HashMap<u64, Account>;
pub type MoneyTransferQueue<'a> = VecDeque<TransferMoneyContext>;

pub struct TransferMoneyContext {
    source_account_id: u64,
    sink_account_id: u64,
    amount: f64,
}

impl TransferMoneyContext {
    pub fn new(source_account_id: u64, sink_account_id: u64, amount: f64) -> Self {
        Self { source_account_id, sink_account_id, amount }
    }

    pub fn execute(
        &mut self,
        accounts: &mut AccountMap,
    ) -> Result<AccountTransaction, AccountTransactionError> {
        let amount = self.amount;

        let source_id = self.source_account_id;

        let source = accounts.get_mut(&source_id).unwrap();

        // withdrawal from source
        if !source.lock() {
            return Err(AccountTransactionError::new(self.source_account_id, format!("Source account#{} is locked", self.source_account_id)));
        }

        let source_balance_after = source.withdraw(amount);

        source.unlock();

        let sink_id = self.sink_account_id;
        let sink = accounts.get_mut(&sink_id).unwrap();

        // deposit to sink
        if !sink.lock() {
            return Err(AccountTransactionError::new(self.sink_account_id, format!("Sink account#{} is locked", self.sink_account_id)));
        }

        let sink_balance_after = sink.deposit(amount);

        sink.unlock();

        Ok(AccountTransaction::new(amount, source_id, source_balance_after, sink_id, sink_balance_after))
    }
}

/// A bank use case using the money transfer context.
pub struct BankContext<'a> {
    accounts: &'a mut AccountMap,
    transfer_queue: &'a mut MoneyTransferQueue<'a>,
}

impl<'a> BankContext<'a> {
    pub fn new(accounts: &'a mut AccountMap, transfer_queue: &'a mut MoneyTransferQueue<'a>) -> Self {
        Self { accounts, transfer_queue }
    }

    pub fn add_transfer(&mut self, transfer: TransferMoneyContext) {
        self.transfer_queue.push_back(transfer);
    }

    pub fn execute_transfers(&mut self) {
        while let Some(mut money_transfer_context) = self.transfer_queue.pop_front() {
            let maybe_tx = money_transfer_context.execute(self.accounts);

            if maybe_tx.is_err() {
                let err = maybe_tx.err().unwrap();
                println!("[error] account#{}: {}", err.account_id, err.message);
                continue;
            }
            
            let tx = maybe_tx.ok().unwrap();

            <BankContext<'_> as AccountOperationLogger>::log(
                &AccountOperation::Withdrawal,
                &tx,
            );

            <BankContext<'_> as AccountOperationLogger>::log(
                &AccountOperation::Deposit, 
                &tx,
            );

            <BankContext<'_> as BankOperationLogger>::log(
                &super::data::BankOperation::TransferMoney, 
                &tx,
            );
        }

    }
}

impl AccountOperationLogger for BankContext<'_> {
    fn log(operation: &AccountOperation, transaction: &AccountTransaction) {
        match operation {
            AccountOperation::Deposit => {
                println!(
                    "[{}] account#{}: {:.6} - {:.6} = {:.6}",
                    operation,
                    transaction.source_account_id(),
                    transaction.source_balance_before(),
                    transaction.amount(),
                    transaction.source_balance_after(),
                );
            }
            AccountOperation::Withdrawal => {
                println!(
                    "[{}] account#{}: {:.6} + {:.6} = {:.6}",
                    operation,
                    transaction.sink_account_id(),
                    transaction.sink_balance_before(),
                    transaction.amount(),
                    transaction.sink_balance_after(),
                );
            }
        }
    }
}

impl BankOperationLogger for BankContext<'_> {
    fn log(operation: &BankOperation, transaction: &AccountTransaction) {
        match operation {
            BankOperation::TransferMoney => {
                println!(
                    "[{}] transferred {:.6} from account#{} -> account#{}",
                    operation,
                    transaction.amount(),
                    transaction.source_account_id(),
                    transaction.sink_account_id(),
                );
            }
        }
    }
}
