use crate::*;

/// Default implementation for Task with empty text list.
impl<'a> Default for Task<'a> {
    fn default() -> Self {
        Self { text_list: vec![] }
    }
}

/// Implementation of task operations.
impl<'a> Task<'a> {
    /// Adds a text configuration to the task list.
    ///
    /// # Arguments
    ///
    /// - `Text` - The text configuration to add
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The task for method chaining
    pub(crate) fn add(&mut self, new_text: Text<'a>) -> &mut Self {
        self.text_list.push(new_text);
        self
    }

    /// Clears all text configurations from the task list.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The cleared task for method chaining
    pub(crate) fn clear(&mut self) -> &mut Self {
        self.text_list.clear();
        self
    }

    /// Runs all tasks in the list.
    ///
    /// # Arguments
    ///
    /// - `&mut Self` - The mutable task instance.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The task instance after execution.
    pub(crate) fn run_all(&mut self) -> &mut Self {
        let copy_task_list: Vec<Text<'a>> = self.text_list.clone();
        self.clear();
        let mut output_str: String = String::new();
        for text in copy_task_list {
            let colored_time: &Cow<'_, str> = &Text::new_from(&text).get_display_str_cow();
            output_str.push_str(colored_time);
        }
        print!("{output_str}");
        std::io::stdout().flush().unwrap();
        self
    }
}
