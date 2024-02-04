use crate::event_predicate::EventPredicate;

// In paper, it is referred as object DAG
enum TimeLineOp<T> {
    // Essentially based on paper, there is only numerical timeline and state dynamic timeline
    // A state dynamic is pretty much state that is dynamic. Consider this as a constant value
    // during the timeline, while numerical keeps moving
    // A numerical timeline essentially cannot be pattern matched, as it is a continuous value
    EqualTo(TimeLineOp<T>, T),
    GreaterThan(TimeLineOp<T>, T),
    LessThan(TimeLineOp<T>, T),
    And(TimeLineOp<T>, TimeLineOp<T>),
    Or(TimeLineOp<T>, TimeLineOp<T>),
    Not(TimeLineOp<T>),
    TlHasExisted(TimeLineOp<T>, EventPredicate),
    TlHasExistedWithin(TimeLineOp<T>, EventPredicate),
    TlLatestEventToState(TimeLineOp<T>, EventPredicate),
    TlDurationWhere(TimeLineOp<T>, EventPredicate),
    TlDurationInCurState(TimeLineOp<T>),
}

impl<T> TimeLineOp<T> {
    fn evaluate(&self) -> TimeLine<T> {
        unimplemented!("evaluate not implemented")
    }
}