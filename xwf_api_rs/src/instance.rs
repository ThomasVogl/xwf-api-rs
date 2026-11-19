use std::any::Any;
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::sync::Mutex;

thread_local! {
    // true while the current thread executes a callback of the X-Tension
    static IN_XT_CALLBACK: Cell<bool> = const { Cell::new(false) };
}

#[derive(Debug, PartialEq, Eq)]
pub enum XtCallError {
    /// X-Ways called a function of the X-Tension while another one is still running in the same
    /// thread (e.g. the X-Tension was started again while its own window pumps window messages)
    Reentrant,
    /// XT_Init has not been called (successfully) before
    NotInitialized,
}

impl Display for XtCallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XtCallError::Reentrant => write!(f, "X-Tension is already running in this thread (reentrant call rejected)"),
            XtCallError::NotInitialized => write!(f, "X-Tension is not initialized"),
        }
    }
}

struct CallbackFlag {}

impl CallbackFlag {
    fn enter() -> Result<CallbackFlag, XtCallError> {
        if IN_XT_CALLBACK.with(|c| c.replace(true)) {
            return Err(XtCallError::Reentrant);
        }
        Ok(CallbackFlag {})
    }
}

impl Drop for CallbackFlag {
    fn drop(&mut self) {
        IN_XT_CALLBACK.with(|c| c.set(false));
    }
}

/// Holds the instance of the X-Tension. Replaces a `static mut`, so that callbacks invoked by
/// X-Ways from several threads (XT_ProcessItem in multi-threaded mode) never access the instance
/// concurrently, and reentrant calls from the same thread are rejected instead of aliasing it.
pub struct XtInstance<T> {
    inner: Mutex<Option<T>>,
}

// access to T is serialized by the mutex
unsafe impl<T> Sync for XtInstance<T> {}

impl<T> XtInstance<T> {
    pub const fn new() -> Self {
        XtInstance { inner: Mutex::new(None) }
    }

    pub fn set(&self, instance: T) -> Result<(), XtCallError> {
        let _flag = CallbackFlag::enter()?;
        *self.inner.lock().unwrap_or_else(|e| e.into_inner()) = Some(instance);
        Ok(())
    }

    pub fn take(&self) -> Result<Option<T>, XtCallError> {
        let _flag = CallbackFlag::enter()?;
        Ok(self.inner.lock().unwrap_or_else(|e| e.into_inner()).take())
    }

    /// Calls `f` with exclusive access to the instance.
    pub fn with<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> Result<R, XtCallError> {
        let _flag = CallbackFlag::enter()?;
        // a panic in a previous call poisons the mutex, the instance is still usable
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let instance = guard.as_mut().ok_or(XtCallError::NotInitialized)?;
        Ok(f(instance))
    }
}

/// Extracts the message of a panic caught via std::panic::catch_unwind()
pub fn panic_message(payload: &Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}
