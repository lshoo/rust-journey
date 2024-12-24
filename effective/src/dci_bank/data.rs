

#[derive(Debug, Clone, Copy)]
pub struct Account {
    pub(crate) id: u64,
    pub(crate) balance: f64,
    pub(crate) locked: bool,
}

impl Account {
    pub fn new(id: u64, balance: f64, locked: bool) -> Self {
        Self { id, balance, locked }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn locked(&self) -> bool {
        self.locked
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountTransaction {
    amount: f64,
    source_account_id: u64,
    source_account_balance: f64,
    sink_account_id: u64,
    sink_account_balance: f64,
}

impl AccountTransaction {
    pub fn new(amount: f64, source_account_id: u64, source_account_balance: f64, sink_account_id: u64, sink_account_balance: f64) -> Self {
        Self { amount, source_account_id, source_account_balance, sink_account_id, sink_account_balance }
    }

    // Getters
    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn source_account_id(&self) -> u64 {
        self.source_account_id
    }

    pub fn sink_account_id(&self) -> u64 {
        self.sink_account_id
    }

    pub fn source_balance_before(&self) -> f64 {
        self.source_account_balance + self.amount
    }

    pub fn source_balance_after(&self) -> f64 {
        self.source_account_balance
    }

    pub fn sink_balance_before(&self) -> f64 {
        self.sink_account_balance - self.amount
    }

    pub fn sink_balance_after(&self) -> f64 {
        self.sink_account_balance
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AccountOperation {
    Deposit,
    Withdrawal,
}

#[derive(Debug, Clone, Copy)]
pub enum BankOperation {
    TransferMoney
}

pub struct AccountTransactionError {
    pub(crate) account_id: u64,
    pub(crate) message: String,
}

impl AccountTransactionError {
    pub fn new(account_id: u64, message: String) -> Self {
        Self { account_id, message }
    }
}