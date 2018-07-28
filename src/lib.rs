use std::{collections, rc};

/// Observer reference collection
///
/// Inteded to be used as an aid in implementing concrete subjects, `Observers` contains a
/// collection of weakly referenced observers that accept events events of type `E` emitted by a
/// subject of type `S`.
///
/// Internally, observers are kept using weak references, any observer that is only referenced
/// by the collection will be freed if there are not other strong references.
///
/// Observers are not guaranteed to be updated in the order they have been registered.
// Note: We actually want to guarantee updates in-order, but cannot currently because `HashMap` is
// used instead of `BTreeMap`.
pub struct Observers<E, S> {
    next_id: usize,
    /// Note: Currently there is a `retain` implementation missing for `HashMap`, it is only
    /// for this reason that a HashMap is used.
    registry: collections::HashMap<usize, rc::Weak<Observer<E, S>>>,
}

impl<E, S> Observers<E, S> {
    /// Create new observer references collection instance.
    #[inline]
    pub fn new() -> Self {
        Observers {
            next_id: 0,
            registry: collections::HashMap::new(),
        }
    }

    /// Notify all currently known observers about `event`.
    ///
    /// During the fan-out of the event, all internal references of dead observers will be cleaned
    /// up.
    #[inline]
    pub fn notify(&mut self, event: E, subject: &S) {
        // `retain` is used to clean up dead observers after trying to call them once.
        self.registry.retain(|_id, obs_ref| {
            if let Some(obs) = obs_ref.upgrade() {
                obs.update(&event, subject);
                true
            } else {
                false
            }
        })
    }

    /// Register an observer.
    ///
    /// Returns a unique ID for the observer that serves as a handle to remove it.
    #[inline]
    pub fn register(&mut self, obs: rc::Weak<Observer<E, S>>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.registry.insert(id, obs);
        id
    }
}

/// Observer of a subject accepting events.
///
/// The observer expects to be updated whenever a subject of type `S` that the observer is
/// registered to emits an event of type `E`.
pub trait Observer<E, S> {
    /// Receive an update.
    ///
    /// Receives an event of type `E` on the subject of type `S`.
    #[inline]
    fn update(&self, event: &E, subject: &S);
}

impl<T, E, S> Observer<E, S> for Box<T>
where
    T: Observer<E, S>,
{
    #[inline]
    fn update(&self, event: &E, subject: &S) {
        (&**self).update(event, subject)
    }
}
