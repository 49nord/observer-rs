use std::rc;

use super::{Observer, ObserverRegistry, Observers};

#[derive(Debug)]
pub enum Event<K> {
    Added(K),
    Remove(K),
    Swapped(K, K),
    Modified(K),
    Reset,
}

macro_rules! observable_list {
    ($name:ident, $col:ty) => {
        #[derive(Debug)]
        pub struct $name<T> {
            inner: $col,
            registry: Observers<Event<usize>, $col>,
        }

        impl<T> $name<T> {
            #[inline]
            pub fn new() -> Self {
                $name {
                    inner: <$col>::new(),
                    registry: Observers::new(),
                }
            }

            #[inline]
            pub fn swap(&mut self, a: usize, b: usize) {
                self.inner.swap(a, b);
                self.registry.notify(Event::Swapped(a, b), &self.inner);
            }

            #[inline]
            pub fn modify<F: Fn(&mut T)>(&mut self, idx: usize, f: F) {
                f(&mut self.inner[idx]);
                self.registry.notify(Event::Modified(idx), &self.inner);
            }

            #[inline]
            pub fn clear(&mut self) {
                self.inner.clear();
                self.registry.notify(Event::Reset, &self.inner);
            }
        }

        impl<T> ObserverRegistry<Event<usize>, $col> for $name<T> {
            #[inline]
            fn register(&mut self, obs: rc::Weak<Observer<Event<usize>, $col>>) -> usize {
                self.registry.register(obs)
            }

            #[inline]
            fn unregister(&mut self, handle: usize) -> bool {
                self.registry.unregister(handle)
            }
        }
    };
}

observable_list!(ObservableVec, Vec<T>);

impl<T> ObservableVec<T> {
    #[inline]
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
        self.registry
            .notify(Event::Added(self.inner.len() - 1), &self.inner);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.inner.len() == 0 {
            return None;
        }

        self.registry
            .notify(Event::Remove(self.inner.len() - 1), &self.inner);
        self.inner.pop()
    }
}

observable_list!(ObservableVecDeque, ::std::collections::VecDeque<T>);

impl<T> ObservableVecDeque<T> {
    #[inline]
    pub fn push_back(&mut self, value: T) {
        self.inner.push_back(value);
        self.registry
            .notify(Event::Added(self.inner.len() - 1), &self.inner);
    }

    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        if self.inner.len() == 0 {
            return None;
        }

        self.registry
            .notify(Event::Remove(self.inner.len() - 1), &self.inner);
        self.inner.pop_back()
    }
}

impl<T> ObservableVecDeque<T> {
    #[inline]
    pub fn push_front(&mut self, value: T) {
        self.inner.push_front(value);
        self.registry
            .notify(Event::Added(self.inner.len() - 1), &self.inner);
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        if self.inner.len() == 0 {
            return None;
        }

        self.registry
            .notify(Event::Remove(self.inner.len() - 1), &self.inner);
        self.inner.pop_front()
    }
}

// #[derive(Debug)]
// pub struct ObservableVec<T> {
//     inner: Vec<T>,
//     registry: Observers<Event<usize>, Vec<T>>,
// }

// impl<T> ObservableVec<T> {
//     #[inline]
//     pub fn new() -> Self {
//         ObservableVec {
//             inner: Vec::new(),
//             registry: Observers::new(),
//         }
//     }

//     #[inline]
//     pub fn push(&mut self, value: T) {
//         self.inner.push(value);
//         self.registry
//             .notify(Event::Added(self.inner.len() - 1), &self.inner);
//     }

//     #[inline]
//     pub fn pop(&mut self) -> Option<T> {
//         if self.inner.len() == 0 {
//             return None;
//         }

//         self.registry
//             .notify(Event::Remove(self.inner.len() - 1), &self.inner);
//         self.inner.pop()
//     }

//     #[inline]
//     pub fn swap(&mut self, a: usize, b: usize) {
//         self.inner.swap(a, b);
//         self.registry.notify(Event::Swapped(a, b), &self.inner);
//     }

//     #[inline]
//     pub fn modify<F: Fn(&mut T)>(&mut self, idx: usize, f: F) {
//         f(&mut self.inner[idx]);
//         self.registry.notify(Event::Modified(idx), &self.inner);
//     }

//     #[inline]
//     pub fn clear(&mut self) {
//         self.inner.clear();
//         self.registry.notify(Event::Reset, &self.inner);
//     }
// }
