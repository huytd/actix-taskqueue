use actix::prelude::*;
use std::marker::PhantomData;
use crossbeam::queue::PopError;

/// Push Message
///
/// The message that will be sent from either producer or the
/// consumer (worker) to enqueue a new task.
///
/// Usage:
/// ```
/// let msg = Push::<i32>::new(10);
/// // or
/// let msg = Push::<&'static str>::new("hello");
/// ```
///
/// Sending this message will put the task to the beginning of
/// the queue.
#[derive(Message)]
#[rtype(result="()")]
pub struct Push<T> {
    pub item: T
}

impl<T> Push<T> {
    pub fn new(value: T) -> Push<T> {
        Push {
            item: value
        }
    }
}

/// Pop Message
///
/// The message that will be sent from consumers (workers) to
/// get the next available task if exists.
///
/// Usage:
/// ```
/// let msg = Pop::<i32>::new();
/// ```
///
/// A `PopError` will be returned if there's no more task available
/// in the queue.
#[derive(Debug, Clone)]
pub struct Pop<T> {
    phantom: PhantomData<T>
}

impl<T> Pop<T> {
    pub fn new() -> Pop<T> {
        Pop {
            phantom: PhantomData
        }
    }
}

impl <T: 'static> Message for Pop<T> {
    type Result = Result<T, PopError>;
}