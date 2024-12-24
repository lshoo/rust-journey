
mod data {
    pub struct Account {
        pub ledger: Vec<f32>,
    }

    impl Account {
        pub fn current_balance(&self) -> f32 {
            self.ledger.iter().sum()
        }
    }
}

mod context {
    pub mod transfer_money {
        use super::super::data;

        pub struct TransferMoney<'a> {
            source: &'a mut data::Account,
            sink: &'a mut data::Account,
            amount: f32,
        }

        impl<'a> TransferMoney<'a> {
            pub fn new(source: &'a mut data::Account, sink: &'a mut data::Account, amount: f32) -> Self {
                Self { source, sink, amount }
            }

            pub fn execute(&mut self) -> () {
                self.source.send_transfer(self.amount, self.sink);
            }
        }

        impl MoneySourceRoleRequirement for data::Account {
            fn available_balance(&self) -> f32 {
                self.current_balance()
            }

            fn decrease_balance(&mut self, amount: f32) -> () {
                self.ledger.push(-amount);
            }
        }

        impl MoneyDestinationRoleRequirement for data::Account {
            fn increase_balance(&mut self, amount: f32) -> () {
                self.ledger.push(amount);
            }
        }

        pub trait MoneySourceRoleRequirement {
            fn available_balance(&self) -> f32;
            fn decrease_balance(&mut self, amount: f32) -> ();
        }

        pub trait MoneySourceRoleMethods: MoneySourceRoleRequirement  {
            fn send_transfer(&mut self, amount: f32, sink: &mut impl MoneyDestinationRoleMethods) -> () {
                if self.available_balance() >= amount {
                    self.decrease_balance(amount);
                    sink.receive_transfer(amount);
                }
            }
        }

        impl<T> MoneySourceRoleMethods for T where T: MoneySourceRoleRequirement {}

        pub trait MoneyDestinationRoleRequirement {
            fn increase_balance(&mut self, amount: f32) -> ();
        }

        pub trait MoneyDestinationRoleMethods: MoneyDestinationRoleRequirement {
            fn receive_transfer(&mut self, amount: f32) -> () {
                self.increase_balance(amount);
            }
        }

        impl<T> MoneyDestinationRoleMethods for T where T: MoneyDestinationRoleRequirement {}
    }
}

pub mod transfer_money_app {
    use super::data;

    use super::context::transfer_money::TransferMoney;

    pub fn run() -> () {
        let mut alice_account = data::Account { ledger: vec![1000.0] };
        let mut bob_account = data::Account { ledger: vec![0.0] };

        println!("Before: ");
        println!("{:?}", alice_account.current_balance());
        println!("{:?}", bob_account.current_balance());

        let mut transfer = TransferMoney::new(&mut alice_account, &mut bob_account, 300.0);
        transfer.execute();

        println!("After: ");
        println!("{:?}", alice_account.current_balance());
        println!("{:?}", bob_account.current_balance());
    }
}
