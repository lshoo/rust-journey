//! https://www.novatec-gmbh.de/en/blog/the-case-for-the-typestate-pattern-the-typestate-pattern-itself/

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RepairOrder<State> {
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub vehicle: String,
    pub customer: Customer,
    pub state: State,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    has_outstanding_debt: bool,
    is_banned: bool,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Employee;

fn find_idle_technician() -> Employee {
    todo!()
}

fn calculate_steps() -> Vec<String> {
    todo!()
}

/// State
#[derive(Debug, Deserialize, Serialize)]
pub struct New;
#[derive(Debug, Deserialize, Serialize)]
pub struct Valid;
#[derive(Debug, Deserialize, Serialize)]
pub struct Invalid {
    validation_errors: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkDone;

#[derive(Debug, Deserialize, Serialize)]
pub struct InProgress {
    assigned_technician: Employee,
    steps_left: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WaitingFroPayment {
    invoice: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Paid {
    invoice: String,
}

impl<State> RepairOrder<State> {
    fn with_state<NewState>(self, new_state: NewState) -> RepairOrder<NewState> {
        RepairOrder {
            order_number: self.order_number,
            damage_description: self.damage_description,
            vehicle: self.vehicle,
            customer: self.customer,
            state: new_state,
        }
    }
}

impl RepairOrder<New> {
    fn validate(self) -> Result<RepairOrder<Valid>, RepairOrder<Invalid>> {
        let is_valid = is_valid();
        if is_valid {
            Ok(self.with_state(Valid))
        } else {
            let validation_errors = get_validation_errors();
            Err(self.with_state(Invalid { validation_errors }))
        }
    }
}

impl RepairOrder<Valid> {
    fn start_progress(
        self,
        technician: Employee,
        steps_left: Vec<String>,
    ) -> RepairOrder<InProgress> {
        self.with_state(InProgress {
            steps_left,
            assigned_technician: technician,
        })
    }
}

impl RepairOrder<InProgress> {
    fn work(mut self) -> RepairOrder<WorkDone> {
        while self.has_step_left() {
            self.work_on_next_step();
        }

        self.with_state(WorkDone)
    }

    fn has_step_left(&self) -> bool {
        self.state.steps_left.is_empty()
    }

    fn work_on_next_step(&mut self) {
        todo!()
    }
}

impl RepairOrder<WorkDone> {
    fn send_invoice(self) -> RepairOrder<WaitingFroPayment> {
        let invoice = get_invoice();
        self.with_state(WaitingFroPayment { invoice })
    }
}

impl RepairOrder<WaitingFroPayment> {
    fn await_payment(self) -> RepairOrder<Paid> {
        let invoice = self.state.invoice.clone();
        await_payment();
        self.with_state(Paid { invoice })
    }
}

pub fn process_fluent(order: RepairOrder<New>) -> Result<RepairOrder<Paid>, RepairOrder<Invalid>> {
    Ok(order
        .validate()?
        .start_progress(find_idle_technician(), calculate_steps())
        .work()
        .send_invoice()
        .await_payment())
}

fn await_payment() {
    todo!()
}

fn get_invoice() -> String {
    todo!()
}

fn get_validation_errors() -> Vec<String> {
    todo!()
}

fn is_valid() -> bool {
    true
}
