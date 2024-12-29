#[test]

fn test_task() {
    use crate::*;
    use task::r#type::Task;
    use text::r#type::Text;
    let mut task: Task<'_> = Task::new();
    task.add(Text::default()).add(Text {
        text: "1",
        ..Text::default()
    });
    task.run_all();
    println!("{:?}", task);
}
