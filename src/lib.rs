use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod tests;

// FIXME: should this be &mut V or &V? split into two traits or drop?
trait Observer<V> {
    fn update(&self, &mut V);
}

// // FIXME: is this a good idea?
// impl<'a, V, T> Observer<V> for &'a T
// where
//     T: Observer<V>,
// {
//     fn update(&self, value: &mut V) {
//         (**self).update(value)
//     }
// }

impl<V, F> Observer<V> for F
where
    F: Fn(&mut V) -> (),
{
    fn update(&self, value: &mut V) {
        // self(value)
    }
}

struct Subject<V, O> {
    observers: Vec<O>,
    value: V,
}

impl<V, O> Subject<V, O>
where
    O: Observer<V>,
{
    fn notify(&mut self) {
        for observer in &mut self.observers {
            observer.update(&mut self.value);
        }
    }
}

struct SubjectGuard<'a, V, O>(&'a mut Subject<V, O>)
where
    V: 'a,
    O: 'a + Observer<V>;

impl<'a, V, O> Drop for SubjectGuard<'a, V, O>
where
    V: 'a,
    O: 'a + Observer<V>,
{
    fn drop(&mut self) {
        &self.0.notify();
    }
}

impl<'a, V, O> Deref for SubjectGuard<'a, V, O>
where
    V: 'a,
    O: 'a + Observer<V>,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.0.value
    }
}

impl<'a, V, O> DerefMut for SubjectGuard<'a, V, O>
where
    V: 'a,
    O: 'a + Observer<V>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.value
    }
}

impl<V, O> Subject<V, O>
where
    O: Observer<V>,
{
    fn new(value: V) -> Subject<V, O> {
        Subject {
            observers: Vec::new(),
            value,
        }
    }

    fn get_mut_notify<'a>(&'a mut self) -> SubjectGuard<'a, V, O> {
        SubjectGuard(self)
    }

    fn into_inner(self) -> V {
        self.value
    }

    fn register(&mut self, observer: O) {
        self.observers.push(observer);
    }
}
