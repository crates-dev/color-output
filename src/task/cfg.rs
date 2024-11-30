use crate::*;
use output_list_builder::r#type::OutputListBuilder;
use task::r#type::{Task, TaskResult};
use text::r#type::Text;

#[test]

fn test_task() {
    let mut task: Task<'_> = Task::new();
    task.add(Text::default()).add(Text {
        text: "1",
        ..Text::default()
    });
    let query_task: TaskResult<'_> = task.query_idx(1);
    let query_task_fail: TaskResult<'_> = task.query_idx(100);
    task.run_all();
    println!("{:?}", task);
}
