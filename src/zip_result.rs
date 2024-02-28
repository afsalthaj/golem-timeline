// A zip result merely represent the result of zipping two timelines
#[derive(Clone, Debug, PartialEq)]
pub enum ZipResult<'t, T: Clone> {
    Both((&'t T, &'t T)),
    Singleton(&'t T),
}

impl<'t, T: Clone> ZipResult<'t, T> {
    pub fn combine(&'t self, other: &'t ZipResult<T>) -> ZipResult<T> {
        match (self, other) {
            (ZipResult::Singleton(a), ZipResult::Singleton(c)) => ZipResult::Both((a, c)),

            // TODO; Return Option
            _ => panic!("Cannot combine"),
        }
    }
}
