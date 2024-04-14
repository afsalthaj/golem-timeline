use crate::bindings::exports::timeline::rawevents::api::{Guest, Order};

mod bindings;

struct Component;


impl Guest for Component {
    fn add_order(order: Order) {
        todo!()
    }

    fn get_events() -> Vec<Order> {
        todo!()
    }
}