use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, Mutex};

use derive_more::{Deref, DerefMut};
use time::OffsetDateTime;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use uuid::{NoContext, Timestamp, Uuid};

/// TODO.
#[derive(Default, Clone)]
pub struct TaskQueue {
    inner: Arc<Mutex<TaskQueueInner>>,
}

#[derive(Default, Deref, DerefMut)]
struct TaskQueueInner {
    tasks: BinaryHeap<TaskData>,
}

/// Represents a single change in task execution status.
#[derive(Debug, Clone)]
pub enum TaskQueueEvent {
    Waiting,
    PreRunning,
    Running,
    PostRunning,
}

impl TaskQueue {
    /// Returns a new [`TaskQueue`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`TaskQueueHandler`].
    #[inline]
    pub fn handler(&self) -> TaskQueueHandler {
        TaskQueueHandler::new(self.clone(), 128)
    }

    /// Adds the task into the task queue.
    fn add_task(&self, tx: Sender<TaskQueueEvent>) -> Uuid {
        let utc_datetime = OffsetDateTime::now_utc();
        let uuid_timestamp = Timestamp::from_unix(
            NoContext,
            utc_datetime.unix_timestamp() as u64,
            utc_datetime.nanosecond(),
        );

        let mut guard = self.inner.lock().expect("should not be held");

        // Makes sure that UUIDv7 is not duplicated.
        let mut task_id = Uuid::new_v7(uuid_timestamp);
        while guard.iter().any(|task| task.id.0 == task_id) {
            task_id = Uuid::new_v7(uuid_timestamp);
        }

        guard.push(TaskData::new(task_id, tx));
        task_id
    }

    /// Removes the task from the task queue.
    pub fn remove_task(&self, id: Uuid) {
        todo!()
    }
}

impl fmt::Debug for TaskQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskQueue").finish_non_exhaustive()
    }
}

/// Contains all the data required to execute a single task.
struct TaskData {
    /// - [`BinaryHeap`] is a max-heap by default, so `cmp::`[`Reverse`] is used.
    /// - UUID `v7` should be used to remain sortable by a timestamp.
    id: Reverse<Uuid>,
    tx: Sender<TaskQueueEvent>,
}

impl TaskData {
    /// Returns a new [`TaskData`].
    #[inline]
    pub fn new(id: Uuid, tx: Sender<TaskQueueEvent>) -> Self {
        Self {
            id: Reverse(id),
            tx,
        }
    }
}

impl PartialEq<Self> for TaskData {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.id, &other.id)
    }
}

impl Eq for TaskData {}

impl PartialOrd<Self> for TaskData {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.id, &other.id)
    }
}

impl Ord for TaskData {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.id, &other.id)
    }
}

/// TODO.
#[derive(Deref, DerefMut)]
pub struct TaskQueueHandler {
    task_queue: TaskQueue,
    send_event: Sender<TaskQueueEvent>,
    task_ids: HashSet<Uuid>,

    #[deref]
    #[deref_mut]
    recv_event: Receiver<TaskQueueEvent>,
}

impl TaskQueueHandler {
    /// Returns a new [`TaskQueueHandler`].
    fn new(task_queue: TaskQueue, channel_cap: usize) -> Self {
        let (tx, rx) = channel::<TaskQueueEvent>(channel_cap);
        Self {
            task_queue,
            send_event: tx.clone(),
            task_ids: HashSet::new(),
            recv_event: rx,
        }
    }

    /// Adds the task into the task queue.
    #[inline]
    pub fn add_task(&self) -> Uuid {
        self.task_queue.add_task(self.send_event.clone())
    }

    /// Removes the task from the task queue.
    #[inline]
    pub fn remove_task(&self, id: Uuid) {
        self.task_queue.remove_task(id)
    }
}

impl fmt::Debug for TaskQueueHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotifyGuard").finish_non_exhaustive()
    }
}

impl Drop for TaskQueueHandler {
    fn drop(&mut self) {
        let inner = &self.task_queue.inner;
        let mut guard = inner.lock().expect("should not be held");
        guard.tasks.retain(|data| {
            let Reverse(id) = data.id;
            !self.task_ids.contains(&id)
        });
    }
}
