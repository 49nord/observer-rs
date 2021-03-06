pub mod collections;
pub mod subject;

use std::{collections::HashMap, fmt, rc};

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
    registry: HashMap<usize, rc::Weak<Observer<E, S>>>,
}

impl<E, S> fmt::Debug for Observers<E, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Observers")
            .field("next_id", &self.next_id)
            .field("registry", &format!("{:?}", self.registry.keys()))
            .finish()
    }
}

impl<E, S> Observers<E, S> {
    /// Create new observer references collection instance.
    #[inline]
    pub fn new() -> Self {
        Observers {
            next_id: 0,
            registry: HashMap::new(),
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
}

impl<E, S> ObserverRegistry<E, S> for Observers<E, S> {
    /// Register an observer.
    ///
    /// Returns a unique ID for the observer that serves as a handle to remove it.
    #[inline]
    fn register(&mut self, obs: rc::Weak<Observer<E, S>>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.registry.insert(id, obs);
        id
    }

    /// Remove an observer from the collection.
    ///
    /// Removes the observer from the internal list of entries. Returns whether or not an entry was
    /// removed.
    #[inline]
    fn unregister(&mut self, handle: usize) -> bool {
        self.registry.remove(&handle).is_some()
    }
}

pub trait ObserverRegistry<E, S> {
    /// Register an observer.
    ///
    /// Returns a unique ID for the observer that serves as a handle to remove it.
    #[inline]
    fn register(&mut self, obs: rc::Weak<Observer<E, S>>) -> usize;

    /// Remove an observer from the collection.
    ///
    /// Removes the observer from the internal list of entries. Returns whether or not an entry was
    /// removed.
    #[inline]
    fn unregister(&mut self, handle: usize) -> bool;
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
