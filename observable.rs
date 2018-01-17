use std::ops::{Deref, DerefMut};

trait Observer<V> {
    fn update(&self, &mut V);
}

// FIXME: is this a good idea?
impl<'a, V, T> Observer<V> for &'a T
where
    T: Observer<V>,
{
    fn update(&self, value: &mut V) {
        (**self).update(value)
    }
}

struct Subject<V, O> {
    observers: Vec<O>,
    value: V,
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
        for observer in &self.0.observers {
            observer.update(&mut self.0.value);
        }
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct Model {
        counter: usize,
    }

    impl Model {
        fn increment(&mut self) {
            self.counter += 1;
        }
    }

    struct TestObserver {
        calls: Cell<usize>,
        last_count: Cell<usize>,
    }

    impl TestObserver {
        fn new() -> TestObserver {
            TestObserver {
                calls: Cell::new(0),
                last_count: Cell::new(0),
            }
        }
    }

    impl Observer<Model> for TestObserver {
        fn update(&self, m: &mut Model) {
            let prev = self.calls.get();
            self.calls.set(prev + 1);

            self.last_count.set(m.counter);
        }
    }

    #[test]
    fn test_subject_wrapping() {
        let model = Model { counter: 42 };

        let mut obs_mod: Subject<Model, TestObserver> = Subject::new(model);

        // let's modify our subject, without an observer
        {
            obs_mod.get_mut_notify().increment();
        }

        // check if the model was modified
        assert_eq!(obs_mod.into_inner().counter, 43);
    }

    #[test]
    fn test_single_observer() {
        let model = Model { counter: 42 };

        // register one observer
        let obs = TestObserver::new();

        let mut obs_mod = Subject::new(model);

        obs_mod.register(&obs);

        {
            obs_mod.get_mut_notify().increment();
        }

        // check if the model was modified
        assert_eq!(obs_mod.into_inner().counter, 43);

        // check if the observer is fine
        assert_eq!(obs.calls.get(), 1);
        assert_eq!(obs.last_count.get(), 43);
    }

    #[test]
    fn test_two_observers() {
        let model = Model { counter: 42 };

        // register one observer
        let obs1 = TestObserver::new();
        let obs2 = TestObserver::new();

        let mut obs_mod = Subject::new(model);

        obs_mod.register(&obs1);
        obs_mod.register(&obs2);

        {
            obs_mod.get_mut_notify().increment();
        }

        // check if the model was modified
        assert_eq!(obs_mod.into_inner().counter, 43);

        // check if the observer is fine
        assert_eq!(obs1.calls.get(), 1);
        assert_eq!(obs1.last_count.get(), 43);

        // check if the observer is fine
        assert_eq!(obs1.calls.get(), 1);
        assert_eq!(obs1.last_count.get(), 43);
    }

    #[test]
    fn test_single_observer_called_twice() {
        let model = Model { counter: 42 };

        // register one observer
        let obs = TestObserver::new();

        let mut obs_mod = Subject::new(model);

        obs_mod.register(&obs);

        {
            obs_mod.get_mut_notify().increment();
            obs_mod.get_mut_notify().increment();
        }

        // check if the model was modified
        assert_eq!(obs_mod.into_inner().counter, 44);

        // check if the observer is fine
        assert_eq!(obs.calls.get(), 2);
        assert_eq!(obs.last_count.get(), 44);
    }

}
