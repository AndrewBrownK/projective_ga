use std::hash::{Hash, Hasher};
use std::sync::Arc;

// Define a wrapper struct around Arc<T>
#[derive(Clone)] // Add Clone if you need to clone the wrapper
pub struct PtrArc<T: ?Sized>(Arc<T>);

// impl<T: ?Sized> From<&Arc<T>> for PtrArc<T> {
//     fn from(arc: &Arc<T>) -> Self {
//         Self(arc.clone())
//     }
// }
// impl<T: ?Sized> From<&PtrArc<T>> for Arc<T> {
//     fn from(value: &PtrArc<T>) -> Self {
//         value.0.clone()
//     }
// }
impl<T: ?Sized> From<Arc<T>> for PtrArc<T> {
    fn from(arc: Arc<T>) -> Self {
        Self(arc)
    }
}
impl<T: ?Sized> From<PtrArc<T>> for Arc<T> {
    fn from(value: PtrArc<T>) -> Self {
        value.0
    }
}


impl<T: ?Sized> Hash for PtrArc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the pointer address, not the value
        (Arc::as_ptr(&self.0) as *const ()).hash(state);
    }
}

impl<T: ?Sized> PartialEq for PtrArc<T> {
    fn eq(&self, other: &Self) -> bool {
        // Use pointer equality, ignoring the value
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: ?Sized> Eq for PtrArc<T> {}