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


/* Minus Three Task Queue */

#[derive(Debug, Default, Clone, Copy)]
struct MinusThree(i32);
struct MinusThreeResult(i32);

#[async_trait]
impl QueueConsumer<MinusThree, MinusThreeResult> for TaskWorker<MinusThree, MinusThreeResult> {
    async fn execute(&self, task: MinusThree) -> Result<MinusThreeResult, WorkerExecuteError> {
        let MinusThree(n) = task;
        if n >= 5 {
            return Ok(MinusThreeResult(n - 3));
        } else if n > 0 {
            return Err(WorkerExecuteError::Retryable);
        } else {
            return Err(WorkerExecuteError::NonRetryable);
        }
    }

    fn get_queue(&self) -> Addr<TaskQueue<MinusThree>> {
        TaskQueue::<MinusThree>::from_registry()
    }

    fn retry(&self, task: MinusThree) -> MinusThree {
        let MinusThree(n) = task;
        println!("MINUS RETRYING VALUE = {}", n);
        MinusThree(n + 1)
    }

    fn drop(&self, task: MinusThree) {
        let MinusThree(n) = task;
        println!("MINUS DROPPED TASK WITH VALUE = {}", n);
    }

    fn result(&self, result: MinusThreeResult) {
        let MinusThreeResult(n) = result;
        println!("MINUS RESULT = {}", n);
    }
}

#[actix_rt::main]
async fn main() {
    let queue = TaskQueue::<PlusFive>::from_registry();
    let worker = TaskWorker::<PlusFive, PlusFiveResult>::new();

    let minus_queue = TaskQueue::<MinusThree>::from_registry();
    let minus_worker = TaskWorker::<MinusThree, MinusThreeResult>::new();

    minus_queue.do_send(Push::new(MinusThree(5)));
    minus_queue.do_send(Push::new(MinusThree(8)));
    minus_queue.do_send(Push::new(MinusThree(3)));
    minus_queue.do_send(Push::new(MinusThree(11)));
    minus_queue.do_send(Push::new(MinusThree(0)));
    minus_queue.do_send(Push::new(MinusThree(20)));


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

    minus_worker.next().await;
    minus_worker.next().await;
    minus_worker.next().await;
    minus_worker.next().await;
    minus_worker.next().await;
    minus_worker.next().await;
    minus_worker.next().await;
    minus_worker.next().await;
    minus_worker.next().await;
    minus_worker.next().await;
}
