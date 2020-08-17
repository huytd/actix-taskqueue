use actix::prelude::*;
use crossbeam::queue::{SegQueue, PopError};
use crate::messages::{Push, Pop};

/// Queueable Trait
///
/// A trait that define the push and pop action to its underlying
/// queue data structure.
pub trait Queueable<T: Default> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Result<T, PopError>;
}

/// TaskQueue Actor
///
/// TaskQueue is an actor that has a queue to hold background Task
/// metadata. It's a passive actor that takes messages from both
/// producer and consumers (workers).
///
/// Messages:
/// - Push:     Add a new task to the queue
/// - Pop:      Take the latest task from the queue
///
/// All tasks will be dispatched in a sequenced manner, that mean
/// there's no support for task priority right now.
///
/// TaskQueue is a SystemService, that mean, it will be automatically
/// started, there will be one service instance per Task type.
///
/// Usage:
/// ```
/// let queue = TaskQueue::<i32>::from_registry();
///
/// queue.do_send(Push::<i32>::new(1));
/// queue.do_send(Push::<i32>::new(2));
/// queue.do_send(Push::<i32>::new(3));
/// ```
#[derive(Default)]
pub struct TaskQueue<T: Default> {
    queue: SegQueue<T>,
}

impl<T: Default> Queueable<T> for TaskQueue<T> {
    fn push(&mut self, item: T) {
        self.queue.push(item);
    }

    fn pop(&mut self) -> Result<T, PopError> {
        self.queue.pop()
    }
}

impl<T: Default + Sized + Unpin + 'static> Actor for TaskQueue<T> {
    type Context = Context<Self>;
}

impl<T: Default + Sized + Unpin + 'static> Handler<Push<T>> for TaskQueue<T> {
    type Result = ();

    fn handle(&mut self, msg: Push<T>, _ctx: &mut Context<Self>) -> Self::Result {
        let item = msg.item;
        self.push(item);
    }
}

impl<T: Default + Sized + Unpin + 'static> Handler<Pop<T>> for TaskQueue<T> {
    type Result = Result<T, PopError>;

    fn handle(&mut self, _: Pop<T>, _ctx: &mut Context<Self>) -> Self::Result {
        self.queue.pop()
    }
}

impl<T: Default + Sized + Unpin + 'static> Supervised for TaskQueue<T> {}
impl<T: Default + Sized + Unpin + 'static> SystemService for TaskQueue<T> {}