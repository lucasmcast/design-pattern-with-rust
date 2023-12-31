use super::{Department, Patient};

#[derive(Default)]
pub struct Cashier {
    next: Option<Box<dyn Department>>
}

impl Department for Cashier {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.payment_done {
            println!("Payment Done");
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}