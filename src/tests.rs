use super::*;
use std::cell::Cell;

#[derive(Debug)]
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

    obs_mod.register(|v: &mut Model| obs.update(v));

    {
        obs_mod.get_mut_notify().increment();
    }

    // // check if the model was modified
    // assert_eq!(obs_mod.into_inner().counter, 43);

    // // check if the observer is fine
    // assert_eq!(obs.calls.get(), 1);
    // assert_eq!(obs.last_count.get(), 43);
}

// #[test]
// fn test_two_observers() {
//     let model = Model { counter: 42 };

//     // register one observer
//     let obs1 = TestObserver::new();
//     let obs2 = TestObserver::new();

//     let mut obs_mod = Subject::new(model);

//     obs_mod.register(&obs1);
//     obs_mod.register(&obs2);

//     {
//         obs_mod.get_mut_notify().increment();
//     }

//     // check if the model was modified
//     assert_eq!(obs_mod.into_inner().counter, 43);

//     // check if the observer is fine
//     assert_eq!(obs1.calls.get(), 1);
//     assert_eq!(obs1.last_count.get(), 43);

//     // check if the observer is fine
//     assert_eq!(obs1.calls.get(), 1);
//     assert_eq!(obs1.last_count.get(), 43);
// }

// #[test]
// fn test_single_observer_called_twice() {
//     let model = Model { counter: 42 };

//     // register one observer
//     let obs = TestObserver::new();

//     let mut obs_mod = Subject::new(model);

//     obs_mod.register(&obs);

//     {
//         obs_mod.get_mut_notify().increment();
//         obs_mod.get_mut_notify().increment();
//     }

//     // check if the model was modified
//     assert_eq!(obs_mod.into_inner().counter, 44);

//     // check if the observer is fine
//     assert_eq!(obs.calls.get(), 2);
//     assert_eq!(obs.last_count.get(), 44);
// }

#[test]
fn test_closure_callbacks() {
    let model = Model { counter: 42 };

    let mut obs_mod = Subject::new(model);

    obs_mod.register(|v: &mut Model| println!("GOT VALUE: {:?}", v));
}
