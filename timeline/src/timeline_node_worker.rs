use std::fmt::Display;
use serde::Serialize;


#[derive(Clone, Debug)]
pub struct TimeLineNodeWorkerInput {
    pub worker_id_prefix: TimeLineWorkerIdPrefix,
    pub template_id: String,
}

#[derive(Clone, Debug)]
pub struct TimeLineWorkerIdPrefix(pub String);

impl Display for TimeLineWorkerIdPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for TimeLineNodeWorkerInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.template_id, self.worker_id_prefix)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TimeLineWorkerId(pub String);

impl Display for TimeLineWorkerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// The worker in which the final execution result of
// the timeline is available. Unlike TimeLineNodeWorkerInput,
// this is a worker-id than just a prefix

#[derive(Clone, Debug, Serialize)]
pub struct TimeLineResultWorker {
    pub worker_id: TimeLineWorkerId,
    pub template_id: String,
}

// This not only says the worker in which result is available,
// but also specifies the type of computation that was done as part of the worker
#[derive(Clone, Debug, Serialize)]
pub enum TypedTimeLineResultWorker {
    LeafTimeLine(LeafTimeLineNode),
    DerivedTimeLine(DerivedTimeLineNode)
}

impl TypedTimeLineResultWorker {
    pub fn tl_has_existed(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::LeafTimeLine(LeafTimeLineNode::TLHasExisted {
            time_line_worker: worker
        })
    }

    pub fn tl_has_existed_within(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::LeafTimeLine(LeafTimeLineNode::TLHasExistedWithin {
            time_line_worker: worker
        })
    }

    pub fn tl_event_to_latest_state(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::LeafTimeLine(LeafTimeLineNode::TLEventToLatestState {
            time_line_worker: worker
        })
    }

    pub fn equal_to(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::DerivedTimeLine(DerivedTimeLineNode::EqualTo {
            result_worker: worker
        })
    }

    pub fn greater_than(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::DerivedTimeLine(DerivedTimeLineNode::GreaterThan {
            result_worker: worker
        })
    }

    pub fn greater_than_or_equal_to(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::DerivedTimeLine(DerivedTimeLineNode::GreaterThanOrEqualTo {
            result_worker: worker
        })
    }

    pub fn less_than(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::DerivedTimeLine(DerivedTimeLineNode::LessThan {
            result_worker: worker
        })
    }

    pub fn less_than_or_equal_to(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::DerivedTimeLine(DerivedTimeLineNode::LessThanOrEqualTo {
            result_worker: worker
        })
    }

    pub fn and(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::DerivedTimeLine(DerivedTimeLineNode::And {
            result_worker: worker,
        })
    }

    pub fn or(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::DerivedTimeLine(DerivedTimeLineNode::Or {
            result_worker: worker,
        })
    }

    pub fn not(worker: TimeLineResultWorker) -> TypedTimeLineResultWorker {
        TypedTimeLineResultWorker::DerivedTimeLine(DerivedTimeLineNode::Not {
            result_worker: worker
        })
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum LeafTimeLineNode {
    TLHasExisted {
        time_line_worker: TimeLineResultWorker,
    },

    TLHasExistedWithin {
        time_line_worker: TimeLineResultWorker,
    },

    TLEventToLatestState {
        time_line_worker: TimeLineResultWorker,
    },
}

#[derive(Clone, Debug, Serialize)]
pub enum DerivedTimeLineNode {
    EqualTo {
        result_worker: TimeLineResultWorker,
    },
    GreaterThan {
        result_worker: TimeLineResultWorker,
    },
    GreaterThanOrEqualTo {
        result_worker: TimeLineResultWorker,
    },
    LessThan {
        result_worker: TimeLineResultWorker,
    },
    LessThanOrEqualTo {
        result_worker: TimeLineResultWorker,
    },

    And {
        result_worker: TimeLineResultWorker,
    },

    Or {
        result_worker: TimeLineResultWorker,
    },

    Not {
        result_worker: TimeLineResultWorker
    },
}