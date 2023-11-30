use tokio::time::{sleep, Duration};

use async_trait::async_trait;

pub trait ServiceInterval { 
    
    fn get_interval_ms(&self) -> u64;
}

// Define a trait that any task will implement
#[async_trait]
pub trait Runnable {
    
    async fn run(&mut self);
 

    async fn start<T: Runnable + 'static + Send + Sync + ServiceInterval>( mut task:  T ) {
        let interval = task.get_interval_ms();
        tokio::spawn(async move {
            loop {
                task.run().await;
                sleep(Duration::from_millis(interval as u64)).await;
            }
        });
    }

}
