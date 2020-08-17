use actix::prelude::*;
use async_trait::async_trait;
use std::marker::PhantomData;
use crate::{messages::{Pop, Push}, queue::{TaskQueue}};

/// WorkerExecuteError Type
///
/// This is the error type that being used to describe an
/// error when it happen inside a worker.
///
/// When the worker running. There are two types of error
/// that would happen:
///
/// - Retryable: Return this when the worker should be retried
/// - NonRetryable: Return this when the worker should not be retried
#[derive(Debug)]
pub enum WorkerExecuteError {
    Retryable,
    NonRetryable
}

/// QueueConsumer Trait
///
/// This is the trait that should be implemented in your application
/// for the pair of Task and TaskResult data structures.
///
/// This trait define the expected behavior of the task queue.
#[async_trait]
pub trait QueueConsumer<T: Default + Sized + Unpin + 'static, W> {
    /// This function takes a task description, and do the work related
    /// to that task. It should return a `Result<TaskResult, WorkerExecuteError>`.
    async fn execute(&self, task: T) -> Result<W, WorkerExecuteError>;

    /// This function returns the `TaskQueue` from the System Registry for
    /// the specific task type that you're having.
    fn get_queue(&self) -> Addr<TaskQueue<T>>;

    /// This function return the new Task object that will be pushed back
    /// to the TaskQueue to be retried.
    fn retry(&self, task: T) -> T;

    /// This function will be called when a Task is dropped from the queue
    /// in case it's not retryable anymore.
    fn drop(&self, task: T);

    /// This function will be called when a Task finished successfully. The
    /// `result` parameter is the returned result of the Task.
    fn result(&self, result: W);
}

pub struct TaskWorker<T: Default + Sized + Unpin + 'static, W: Send + Sized + Unpin + 'static> {
    task: PhantomData<T>,
    result: PhantomData<W>
}

impl<T: Default + Sized + Unpin + 'static, W: Send + Sized + Unpin + 'static> Actor for TaskWorker<T, W> {
    type Context = Context<Self>;
}

impl<T: Copy + Default + Sized + Unpin + Send + 'static, W: Send + Sized + Unpin + 'static> TaskWorker<T, W> where Self: QueueConsumer<T, W> {
    pub fn new() -> TaskWorker<T, W> {
        TaskWorker {
            task: PhantomData,
            result: PhantomData
        }
    }

    /// Call this function when the Worker is ready to process the next task.
    pub async fn next(&self) {
        let queue = self.get_queue();
        if let Ok(ret) = queue.send(Pop::new()).await {
            match ret {
                Ok(task) => {
                    let execute_result = self.execute(task).await;
                    match execute_result {
                        Ok(result) => {
                            self.result(result);
                        },
                        Err(err) => {
                            match err {
                                WorkerExecuteError::Retryable => queue.do_send(Push::new(self.retry(task))),
                                WorkerExecuteError::NonRetryable => self.drop(task)
                            }
                        }
                    }
                }
                Err(error) => println!("{:?}", error)
            }
        }

    }
}