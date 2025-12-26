#[test]

fn test_task() {
    use crate::*;
    use task::r#struct::Task;
    use text::r#struct::Text;
    let mut task: Task<'_> = Task::default();
    task.add(Text::default()).add(Text {
        text: "1",
        ..Text::default()
    });
    task.run_all();
    println!("{task:?}");
}
