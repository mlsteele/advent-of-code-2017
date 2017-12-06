use std;

const WRAP_INVARIANT_MSG: &str = "wrap invariant violated: holds no value";

#[derive(Debug)]
pub struct Wrap<T> {
    // This must always be Some except in the `mutate` method.
    inner: Option<T>
}

impl<T> Wrap<T> {
    pub fn new(x: T) -> Self {
        Self{
            // inner: Cell::new(Some(x))
            inner: Some(x)
        }
    }

    pub fn mutate<F>(&mut self, f: F)
        where F: FnOnce(T) -> T
    {
        let prev = std::mem::replace(&mut self.inner, None).expect(WRAP_INVARIANT_MSG);
        self.inner = Some(f(prev));
    }

    #[allow(dead_code)]
    pub fn to_inner(self) -> T {
        self.inner.expect(WRAP_INVARIANT_MSG)
    }
}

impl<T> AsRef<T> for Wrap<T> {
    fn as_ref(&self) -> &T {
        match self.inner {
            Some(ref x) => x,
            None => {
                panic!(WRAP_INVARIANT_MSG)
            }
        }
    }
}


impl<T> AsMut<T> for Wrap<T> {
    fn as_mut(&mut self) -> &mut T {
        match self.inner {
            Some(ref mut x) => x,
            None => {
                panic!(WRAP_INVARIANT_MSG)
            }
        }
    }
}
