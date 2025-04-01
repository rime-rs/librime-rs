use std::any::Any;
use std::collections::VecDeque;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

type TaskInitializer = Box<dyn Any + Send + Sync>;

trait DeploymentTask: Send + Sync {
    fn run<'a>(&'a self, deployer: &'a Deployer)
    -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>;
}

struct Deployer {
    shared_data_dir: PathBuf,
    user_data_dir: PathBuf,
    prebuilt_data_dir: PathBuf,
    staging_dir: PathBuf,
    sync_dir: PathBuf,
    user_id: String,
    distribution_name: String,
    distribution_code_name: String,
    distribution_version: String,
    app_name: String,
    pending_tasks: Arc<Mutex<VecDeque<Box<dyn DeploymentTask>>>>,
    work: Option<JoinHandle<()>>,
    maintenance_mode: bool,
}

impl Deployer {
    fn new() -> Self {
        Deployer {
            shared_data_dir: PathBuf::from("."),
            user_data_dir: PathBuf::from("."),
            prebuilt_data_dir: PathBuf::from("build"),
            staging_dir: PathBuf::from("build"),
            sync_dir: PathBuf::from("sync"),
            user_id: String::from("unknown"),
            distribution_name: String::new(),
            distribution_code_name: String::new(),
            distribution_version: String::new(),
            app_name: String::new(),
            pending_tasks: Arc::new(Mutex::new(VecDeque::new())),
            work: None,
            maintenance_mode: false,
        }
    }

    async fn run_task(&self, task_name: &str, arg: TaskInitializer) -> bool {
        // Placeholder for task creation and execution
        // In Rust, you would typically use a factory pattern or a match statement
        // to create tasks based on the task_name.
        // For simplicity, we'll assume a task is created and run here.
        let task: Box<dyn DeploymentTask> = Box::new(ExampleTask);
        task.run(self).await
    }

    async fn schedule_task(&self, task: Box<dyn DeploymentTask>) {
        let mut tasks = self.pending_tasks.lock().await;
        tasks.push_back(task);
    }

    async fn next_task(&self) -> Option<Box<dyn DeploymentTask>> {
        let mut tasks = self.pending_tasks.lock().await;
        tasks.pop_front()
    }

    async fn has_pending_tasks(&self) -> bool {
        let tasks = self.pending_tasks.lock().await;
        !tasks.is_empty()
    }

    async fn run(&self) -> bool {
        println!("running deployment tasks:");
        let mut success = 0;
        let mut failure = 0;
        loop {
            while let Some(task) = self.next_task().await {
                let result = task.run(self).await;
                if result {
                    success += 1;
                } else {
                    failure += 1;
                }
            }
            println!(
                "{} tasks ran: {} success, {} failure.",
                success + failure,
                success,
                failure
            );
            if !self.has_pending_tasks().await {
                break;
            }
        }
        failure == 0
    }

    async fn start_work(&mut self, maintenance_mode: bool) -> bool {
        if self.is_working() {
            println!("a work thread is already running.");
            return false;
        }
        self.maintenance_mode = maintenance_mode;
        if self.pending_tasks.lock().await.is_empty() {
            return false;
        }
        let tasks = Arc::clone(&self.pending_tasks);
        let deployer = Deployer {
            pending_tasks: tasks,
            ..Deployer::new()
        };
        self.work = Some(tokio::spawn(async move {
            deployer.run().await;
        }));
        true
    }

    async fn start_maintenance(&mut self) -> bool {
        self.start_work(true).await
    }

    fn is_working(&self) -> bool {
        if let Some(handle) = &self.work {
            !handle.is_finished()
        } else {
            false
        }
    }

    fn is_maintenance_mode(&self) -> bool {
        self.maintenance_mode && self.is_working()
    }

    async fn join_work_thread(&mut self) {
        if let Some(handle) = self.work.take() {
            handle.await.unwrap();
        }
    }

    async fn join_maintenance_thread(&mut self) {
        self.join_work_thread().await;
    }

    fn user_data_sync_dir(&self) -> PathBuf {
        self.sync_dir.join(&self.user_id)
    }
}

struct ExampleTask;

impl DeploymentTask for ExampleTask {
    fn run<'a>(
        &'a self,
        _deployer: &'a Deployer,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>> {
        Box::pin(async move {
            println!("ExampleTask is running.");
            true
        })
    }
}

// #[tokio::main]
// async fn main() {
//     let mut deployer = Deployer::new();
//     deployer.schedule_task(Box::new(ExampleTask)).await;
//     deployer.start_work(false).await;
//     deployer.join_work_thread().await;
// }
