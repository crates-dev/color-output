use crate::*;

#[test]

fn test_task() {
    let mut task: Task<'_> = Task::default();
    task.add(Text::default()).add(Text {
        text: "1",
        ..Text::default()
    });
    task.run_all();
    println!("{task:?}");
}
