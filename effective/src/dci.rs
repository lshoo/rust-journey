
/// DCI (Data, Context, Interaction) Program Paradigm
/// Data
/// Data represents what the system IS
pub mod data {
    
    use crate::dci::context::transfer_money::MoneySourceRoleRequirement;
    use crate::dci::context::transfer_money::MoneyDestinationRoleRequirement;
    // Account is an object that keeps a record for its transactions
    pub struct Account {
        pub ledger: Vec<f32>,
    }

    impl Account {
        pub fn current_balance(&self) -> f32 {
            self.ledger.iter().sum()
        }
    }

    macro_rules! impl_trait_for {
        ($type_:ty; $trait:ident; {
            $($each_implementation:tt)*
        }) => {
            impl $type_ {
                pub $($each_implementation)*
            }
            impl $trait for $type_ {
                $($each_implementation)*
            }
        };
    }

    // the macro call below implements the methods for the MoneySourceRoleRequirement trait
    // and also adds the following:
    /*
        impl Account {
            pub fn available_balance(&self) -> f32 {
                self.current_balance()
            }
    
            pub fn decrease_balance(&mut self, amount: f32) -> () {
                self.ledger.push(-amount);
            }
        }
    */
    impl_trait_for!(Account; MoneySourceRoleRequirement; {
        fn available_balance(&self) -> f32 {
            self.current_balance()
        }

        fn decrease_balance(&mut self, amount: f32) -> () {
            self.ledger.push(-amount);
        }
    });

    impl_trait_for!(Account; MoneyDestinationRoleRequirement; {
        fn increase_balance(&mut self, amount: f32) -> () {
            self.ledger.push(amount);
        }
    });
}

/// Context
/// Context represents what the system DOES
pub mod context {
    // CONTEXT: transfer money.
    // A specification of money transfer use case
    pub mod transfer_money {
        // Declare that this use case will use and depend on Data
        use crate::dci::data;

        // As with transer money in the real life, there are two important roles involved in this use:
        // 1. Source
        // 2. Destination
        // The algorithm is simple transfer a specified amount of money from object that plays Money Source role 
        // to the object that plays Money Destination role.

        // Role MoneySource
        // The contract/Requirement below must be fullfilled for object that will play the role of MoneySource
        pub trait MoneySourceRoleRequirement {
            fn available_balance(&self) -> f32;
            fn decrease_balance(&mut self, amount: f32) -> ();
        }

        trait MoneySourceRoleMethods: MoneySourceRoleRequirement {
            fn send_transfer(&mut self, amount: f32, sink: &mut dyn MoneyDestinationRoleMethods) -> () {
                if self.available_balance() >= amount {
                    self.decrease_balance(amount);
                    sink.receive_transfer(amount);
                }
            }
        }

        impl<T> MoneySourceRoleMethods for T where T: MoneySourceRoleRequirement { }

        // Role MoneyDestination
        pub trait MoneyDestinationRoleRequirement {
            fn increase_balance(&mut self, amount: f32) -> ();
        }

        trait MoneyDestinationRoleMethods: MoneyDestinationRoleRequirement {
            fn receive_transfer(&mut self, amount: f32) -> () {
                self.increase_balance(amount);
            }
        }

        impl<T> MoneyDestinationRoleMethods for T where T: MoneyDestinationRoleRequirement { }

        pub struct TransferMoney<'a> {
            source: &'a mut data::Account,
            destination: &'a mut data::Account,
            amount: f32,
        }

        impl<'a> TransferMoney<'a> {
            pub fn new(source: &'a mut data::Account, destination: &'a mut data::Account, amount: f32) -> Self {
                Self { source, destination, amount }
            }

            pub fn execute(&mut self) -> () {
                self.source.send_transfer(self.amount, self.destination);
            }
        }

    }
}

pub mod transfer_money_app {
    use crate::dci::context::transfer_money::TransferMoney;
    use crate::dci::data;

    pub fn run() {
        let alice_account = &mut data::Account { ledger: vec![1000_f32] };
        let bob_account = &mut data::Account { ledger: vec![100_f32] };

        println!("Before: ");
        println!("Alice: {}", alice_account.current_balance());
        println!("Bob: {}", bob_account.current_balance());

        {
            let mut context = TransferMoney::new(alice_account, bob_account, 300_f32);
            context.execute();
        }

        println!("After: ");
        println!("Alice: {}", alice_account.current_balance());
        println!("Bob: {}", bob_account.current_balance());
    }
}