use crate::*;

impl<'a> Default for Task<'a> {
    fn default() -> Self {
        Self { text_list: vec![] }
    }
}

impl<'a> Task<'a> {
    /// Adds a text structure to the task list.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current task instance.
    /// - `new_text`: The text structure to be added.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current task instance, allowing method chaining.
    ///
    /// If the `new_text` is empty, no text is added and the method returns the current task instance unchanged.
    pub(crate) fn add(&mut self, new_text: Text<'a>) -> &mut Self {
        if new_text.text.is_empty() {
            return self;
        }
        self.text_list.push(new_text);
        self
    }

    /// Removes a task at the specified index.
    ///
    /// # Parameters
    /// - `&mut self`: The current task instance
    /// - `idx`: The index of the task to be removed
    ///
    /// # Returns
    /// - `TaskResult`: The result of the removal operation
    pub(crate) fn remove(&mut self, idx: usize) -> TaskResult {
        if idx >= self.text_list.len() {
            return TaskResult::Fail;
        }
        self.text_list.remove(idx);
        TaskResult::SuccessDefault
    }

    /// Queries a task at the specified index.
    ///
    /// # Parameters
    /// - `&mut self`: The current task instance
    /// - `idx`: The index of the task to be queried
    ///
    /// # Returns
    /// - `TaskResult`: The result of the query
    pub(crate) fn query_idx(&mut self, idx: usize) -> TaskResult {
        if idx >= self.text_list.len() {
            return TaskResult::Fail;
        }
        let text: Text<'a> = self.text_list[idx].clone();
        let remove_res: TaskResult<'_> = self.remove(idx);
        match remove_res {
            TaskResult::SuccessDefault => {
                let output_str: Cow<'_, str> = Text::new_from(&text).get_display_str_cow();
                return TaskResult::SuccessText(text);
            }
            _ => remove_res,
        }
    }

    /// Queries a task at the specified index and formats it into a string.
    ///
    /// # Parameters
    /// - `&mut self`: The current task instance
    /// - `idx`: The index of the task to be queried
    ///
    /// # Returns
    /// - `TaskResult`: The result of the query in string format
    pub(crate) fn query_idx_format_str(&mut self, idx: usize) -> TaskResult {
        if idx >= self.text_list.len() {
            return TaskResult::Fail;
        }
        let text: Text<'a> = self.text_list[idx].clone();
        self.text_list.remove(idx);
        let output_str: String = Text::new_from(&text).get_display_str_cow().into_owned();
        TaskResult::SuccessStr(output_str)
    }

    /// Runs the task at the specified index.
    ///
    /// # Parameters
    /// - `&mut self`: The current task instance
    /// - `idx`: The index of the task to be run
    ///
    /// # Returns
    /// - `TaskResult`: The result of the task run
    pub(crate) fn run_idx(&mut self, idx: usize) -> TaskResult {
        let result: TaskResult<'_> = self.query_idx(idx);
        if result == TaskResult::Fail {
            return TaskResult::Fail;
        }
        match result {
            TaskResult::SuccessText(success_text) => {
                let output_str: Cow<'_, str> = Text::new_from(&success_text).get_display_str_cow();
                print!("{}", output_str);
                TaskResult::SuccessText(success_text)
            }
            _ => result,
        }
    }

    /// Clears all tasks from the task list.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current task instance.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current task instance, allowing method chaining.
    ///
    /// This method removes all tasks from the task list.
    pub(crate) fn clear(&mut self) -> &mut Self {
        self.text_list.clear();
        self
    }

    /// Runs all tasks and outputs the result as a string.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current task instance.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current task instance, allowing method chaining.
    ///
    /// The method clones the task list, clears the original list, and then processes each task by
    /// converting its output to a string and printing the result.
    pub(crate) fn run_all(&mut self) -> &mut Self {
        let copy_task_list: Vec<Text<'a>> = self.text_list.clone();
        self.clear();
        let mut output_str: String = String::new();
        for text in copy_task_list {
            let colored_time: &Cow<'_, str> = &Text::new_from(&text).get_display_str_cow();
            output_str.push_str(colored_time);
        }
        print!("{}", output_str);
        std::io::stdout().flush().unwrap();
        self
    }
}
