use std::ops::Deref;

// A zip result merely represent the result of zipping two timelines
#[derive(Clone, Debug, PartialEq)]
pub enum ZipResult<'t, T: Clone> {
    Both((&'t T, Box<ZipResult<'t, T>>)),
    Singleton(&'t T, Side),
}


#[derive(Clone, Debug, PartialEq)]
pub enum Side  {
    Left,
    Right,
}

impl<'t, T: Clone + Sized> ZipResult<'t, T> {
    pub fn merge<F>(&'t self, f: &F) -> T where F: Fn(&T, &T) -> T {
        fn go<'t, B, G>(prev: Option<&'t B>, zip_result: &ZipResult<'t, B>, f: &G) -> B where G: Fn(&B, &B) -> B, B: Clone {
            match zip_result {
                ZipResult::Singleton(a, side) =>
                    if let Some(prev) = prev {
                        match side {
                            Side::Left => f(a, prev),
                            Side::Right => f(prev, a),
                        }
                    } else {
                        a.deref().clone()
                    },
                ZipResult::Both((a, b)) => go(Some(a), b, f)
            }
        }

        go(None, self, f)

    }


}
