use crate::backend::BackEnd;
use crate::timeline_op::TimeLineOp;
use crate::worker_timeline::WorkerKey;
use crate::worker_timeline_data::InvokeWorker;
use futures::StreamExt;
use futures::{stream, Stream};

pub trait TimeLineExecution {
    fn run(&self) -> WorkerId;
}

pub struct WorkerId(pub String);

impl TimeLineExecution for TimeLineOp {
    fn run(&self) -> WorkerId {
        match self {
            TimeLineOp::Leaf() => panic!("Not implemented"),
            TimeLineOp::EqualTo(_, _) =>panic!("Not implemented"),
            TimeLineOp::GreaterThan(_, _) =>panic!("Not implemented"),
            TimeLineOp::GreaterThanOrEqual(_, _) =>panic!("Not implemented"),
            TimeLineOp::LessThan(_, _) =>panic!("Not implemented"),
            TimeLineOp::LessThanOrEqual(_, _) =>panic!("Not implemented"),
            TimeLineOp::And(_, _) =>panic!("Not implemented"),
            TimeLineOp::Or(_, _) =>panic!("Not implemented"),
            TimeLineOp::Not(_) =>panic!("Not implemented"),
            TimeLineOp::TlHasExisted(_, _) =>panic!("Not implemented"),
            TimeLineOp::TlHasExistedWithin(_, _, _) =>panic!("Not implemented"),
            TimeLineOp::TlLatestEventToState(_, _) =>panic!("Not implemented"),
            TimeLineOp::TlDurationWhere(_, _) =>panic!("Not implemented"),
            TimeLineOp::TlDurationInCurState(_, _) =>panic!("Not implemented")
        }
    }
}
