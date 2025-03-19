use std::collections::HashMap;
use std::fmt::Debug;

use crate::bindings::exports::timeline::event_processor_exports::api::{
    Event, EventPredicate, EventPredicateOp, EventValue,
};
use common_lib::{EventColumnName, EventColumnValue, GolemEventPredicate};
use common_lib::{GolemEvent, GolemEventValue};

pub trait Conversion: Clone + Debug {
    type WitType: Clone;
    fn from_wit(input: Self::WitType) -> Self;
    fn to_wit(&self) -> Self::WitType;
}

impl Conversion for GolemEventValue {
    type WitType = EventValue;

    fn from_wit(input: Self::WitType) -> Self {
        match input {
            EventValue::StringValue(s) => GolemEventValue::StringValue(s.clone()),
            EventValue::IntValue(i) => GolemEventValue::IntValue(i),
            EventValue::FloatValue(fl) => GolemEventValue::FloatValue(fl),
            EventValue::BoolValue(b) => GolemEventValue::BoolValue(b),
        }
    }

    fn to_wit(&self) -> Self::WitType {
        match self {
            GolemEventValue::StringValue(s) => EventValue::StringValue(s.clone()),
            GolemEventValue::IntValue(i) => EventValue::IntValue(*i),
            GolemEventValue::FloatValue(fl) => EventValue::FloatValue(*fl),
            GolemEventValue::BoolValue(b) => EventValue::BoolValue(*b),
        }
    }
}

impl Conversion for GolemEventPredicate<GolemEventValue> {
    type WitType = EventPredicate;

    fn from_wit(input: Self::WitType) -> Self {
        let event_column = EventColumnName(input.col_name.clone());
        let event_value = EventColumnValue::from(GolemEventValue::from_wit(input.value.clone()));
        match input.op {
            EventPredicateOp::Equal => GolemEventPredicate::Equals(event_column, event_value),
            EventPredicateOp::GreaterThan => {
                GolemEventPredicate::GreaterThan(event_column, event_value)
            }
            EventPredicateOp::LessThan => GolemEventPredicate::LessThan(event_column, event_value),
        }
    }

    fn to_wit(&self) -> Self::WitType {
        match self {
            GolemEventPredicate::Equals(event_column, event_value) => EventPredicate {
                col_name: event_column.0.clone(),
                value: event_value.0.to_wit(),
                op: EventPredicateOp::Equal
            },
            GolemEventPredicate::GreaterThan(event_column, event_value) => EventPredicate {
                col_name: event_column.0.clone(),
                value: event_value.0.to_wit(),
                op: EventPredicateOp::GreaterThan
            },
            GolemEventPredicate::LessThan(event_column, event_value) => EventPredicate {
                col_name: event_column.0.clone(),
                value: event_value.0.to_wit(),
                op: EventPredicateOp::LessThan
            },
            _ => panic!("Not all possible event predicate represented in WIT. This will be included in near future")
        }
    }
}

impl Conversion for GolemEvent<GolemEventValue> {
    type WitType = Event;

    fn from_wit(input: Self::WitType) -> Self {
        let mut event = GolemEvent { time: input.time, event: HashMap::new() };

        for (key, value) in input.event {
            event.event.insert(EventColumnName(key), GolemEventValue::from_wit(value));
        }

        event
    }

    fn to_wit(&self) -> Self::WitType {
        let mut event = Event { time: self.time, event: vec![] };

        for (key, value) in self.event.iter() {
            event.event.push((key.0.clone(), value.to_wit()));
        }

        event
    }
}