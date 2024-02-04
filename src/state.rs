use crate::timeline::TimeLine;
struct State<T> {
    hash: u64,
    time_line: TimeLine<T>,
}

impl<T> State<T> {
    fn new(hash: u64, time_line: TimeLine<T>) -> State<T> {
        State {
            hash,
            time_line,
        }
    }
}