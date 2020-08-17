use async_trait::async_trait;
use actix::prelude::*;
use actix_taskqueue::queue::{TaskQueue};
use actix_taskqueue::{messages::Push, worker::{QueueConsumer, TaskWorker, WorkerExecuteError}};

/* Plus Five Task Queue */

#[derive(Debug, Default, Clone, Copy)]
struct PlusFive(i32);
struct PlusFiveResult(i32);

#[async_trait]
impl QueueConsumer<PlusFive, PlusFiveResult> for TaskWorker<PlusFive, PlusFiveResult> {
    async fn execute(&self, task: PlusFive) -> Result<PlusFiveResult, WorkerExecuteError> {
        let PlusFive(n) = task;
        if n >= 5 {
            return Ok(PlusFiveResult(n + 5));
        } else if n > 0 {
            return Err(WorkerExecuteError::Retryable);
        } else {
            return Err(WorkerExecuteError::NonRetryable);
        }
    }

    fn get_queue(&self) -> Addr<TaskQueue<PlusFive>> {
        TaskQueue::<PlusFive>::from_registry()
    }

    fn retry(&self, task: PlusFive) -> PlusFive {
        let PlusFive(n) = task;
        println!("RETRYING VALUE = {}", n);
        PlusFive(n + 1)
    }

    fn drop(&self, task: PlusFive) {
        let PlusFive(n) = task;
        println!("DROPPED TASK WITH VALUE = {}", n);
    }

    fn result(&self, result: PlusFiveResult) {
        let PlusFiveResult(n) = result;
        println!("RESULT = {}", n);
    }
}

#[actix_rt::main]
async fn main() {
    let queue = TaskQueue::<PlusFive>::from_registry();
    let worker = TaskWorker::<PlusFive, PlusFiveResult>::new();

    queue.do_send(Push::new(PlusFive(5)));
    queue.do_send(Push::new(PlusFive(8)));
    queue.do_send(Push::new(PlusFive(3)));
    queue.do_send(Push::new(PlusFive(11)));
    queue.do_send(Push::new(PlusFive(0)));
    queue.do_send(Push::new(PlusFive(20)));

    worker.next().await;
    worker.next().await;
    worker.next().await;
    worker.next().await;
    worker.next().await;
    worker.next().await;
    worker.next().await;
    worker.next().await;
    worker.next().await;
    worker.next().await;
}
