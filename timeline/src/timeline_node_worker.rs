use std::fmt::Display;

pub struct WorkerId(pub String);

#[derive(Clone, Debug)]
pub struct TimeLineNodeWorker {
    pub worker_id: String,
    pub template_id: String,
}

impl Display for TimeLineNodeWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.template_id, self.worker_id)
    }
}
