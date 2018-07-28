use super::{ObserverRegistry, Observers};

pub type Changed = ();

#[derive(Debug)]
pub struct Subject<T> {
    inner: T,
    observers: Observers<Changed, T>,
}

impl<T> Subject<T> {
    #[inline]
    pub fn new(inner: T) -> Subject<T> {
        Subject {
            inner,
            observers: Observers::new(),
        }
    }

    #[inline]
    pub fn get(&self) -> &T {
        &self.inner
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }

    #[inline]
    pub unsafe fn maybe_mutate<F>(&mut self, f: F)
    where
        F: Fn(&mut T) -> bool,
    {
        if f(&mut self.inner) {
            // Change reported, update all of our registered observers:
            self.observers.notify((), &self.inner);
        }
    }

    #[inline]
    pub fn mutate<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        unsafe {
            self.maybe_mutate(|v| {
                f(v);
                true
            })
        }
    }

    #[inline]
    pub fn observers(&mut self) -> &mut ObserverRegistry<Changed, T> {
        // We return the trait reference instead of the observers collection to block access to
        // `notify()`.
        &mut self.observers
    }
}
