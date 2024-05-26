use crate::bindings::timeline::event_processor::api::{
    EventValue, TimePeriod, TimelineResult, TimelineResultPoint,
};
use std::fmt::Debug;
use timeline::*;

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

impl Conversion for StateDynamicsTimeLinePoint<GolemEventValue> {
    type WitType = TimelineResultPoint;

    fn from_wit(input: Self::WitType) -> Self {
        StateDynamicsTimeLinePoint {
            t1: input.time_period.t1,
            t2: input.time_period.t2,
            value: GolemEventValue::from_wit(input.value),
        }
    }

    fn to_wit(&self) -> Self::WitType {
        TimelineResultPoint {
            time_period: TimePeriod { t1: self.t1, t2: self.t2 },
            value: self.value.to_wit(),
        }
    }
}

impl Conversion for StateDynamicsTimeLine<GolemEventValue> {
    type WitType = TimelineResult;

    fn from_wit(input: Self::WitType) -> Self {
        StateDynamicsTimeLine::from(
            input
                .results
                .iter()
                .map(|point| StateDynamicsTimeLinePoint::from_wit(point.clone()))
                .collect(),
        )
    }

    fn to_wit(&self) -> Self::WitType {
        TimelineResult { results: self.points.iter().map(|point| point.1.to_wit()).collect() }
    }
}
