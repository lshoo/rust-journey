

extern crate log;

use effective::dci_bank::data::Account;
use effective::dci_bank::context::{BankContext, AccountMap, MoneyTransferQueue, TransferMoneyContext};

fn main() {
    pretty_env_logger::init();

    let account1_id = 1111u64;
    let account2_id = 2222u64;
    let account3_id = 3333u64;

    let mut accounts = AccountMap::new();
    accounts.insert(account1_id, Account::new(account1_id, 10000.0, false));
    accounts.insert(account2_id, Account::new(account2_id, 2000.0, false));
    accounts.insert(account3_id, Account::new(account3_id, 3000.0, false));

    let mut transfer_queue = MoneyTransferQueue::new();

    let mut bank = BankContext::new(&mut accounts, &mut transfer_queue);

    let tx1 = TransferMoneyContext::new(account1_id, account2_id, 4200.0);
    let tx2 = TransferMoneyContext::new(account2_id, account3_id, 3200.0);
    let tx3 = TransferMoneyContext::new(account3_id, account1_id, 1200.0);

    bank.add_transfer(tx1);
    bank.add_transfer(tx2);
    bank.add_transfer(tx3);

    bank.execute_transfers();

    println!("Accounts: {:?}", accounts);
}
