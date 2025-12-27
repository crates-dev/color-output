use crate::*;

impl<'a> Default for OutputListBuilder<'a> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> OutputListBuilder<'a> {
    /// Creates a new empty OutputListBuilder.
    ///
    /// # Returns
    ///
    /// - `OutputListBuilder` - New instance with empty output list
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            output_list: vec![],
        }
    }

    /// Creates a new OutputListBuilder from existing outputs.
    ///
    /// # Arguments
    ///
    /// - `Vec<Output>` - Collection of outputs to initialize with
    ///
    /// # Returns
    ///
    /// - `OutputListBuilder` - New instance containing the specified outputs
    #[inline(always)]
    pub fn new_from(output_list: Vec<Output<'a>>) -> Self {
        Self { output_list }
    }

    /// Adds an output to the list.
    ///
    /// # Arguments
    ///
    /// - `Output` - The output configuration to add
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for method chaining
    #[inline(always)]
    pub fn add(&mut self, output: Output<'a>) -> &mut Self {
        self.output_list.push(output);
        self
    }

    /// Removes an output item from the list at the specified index.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of `OutputListBuilder`.
    /// - `idx`: The index of the output item to be removed.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current instance, allowing for method chaining.
    ///
    /// If the index is out of bounds, the list remains unchanged.
    pub fn remove(&mut self, idx: usize) -> &mut Self {
        if idx >= self.output_list.len() {
            return self;
        }
        self.output_list.remove(idx);
        self
    }

    /// Clears all output items from the output list.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of `OutputListBuilder`.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.output_list.clear();
    }

    /// Runs all output items in the list, executing their output logic.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of `OutputListBuilder`.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current instance, allowing for method chaining.
    ///
    /// The method clones the current output list, clears the original list, and executes
    /// the output for each cloned item.
    pub fn run(&mut self) -> &mut Self {
        let outputs: Vec<Output<'_>> = self.output_list.to_vec();
        self.clear();
        output_list(&outputs);
        self
    }

    /// Queries the output item at the specified index.
    ///
    /// # Parameters
    /// - `&self`: An immutable reference to the current instance of `OutputListBuilder`.
    /// - `idx`: The index of the output item to query.
    ///
    /// # Returns
    /// - `Output`: The output item at the specified index, or a default output if the index is out of bounds.
    pub fn query_idx(&'_ self, idx: usize) -> Output<'_> {
        if idx >= self.output_list.len() {
            return Output::default();
        }
        self.output_list[idx]
    }

    /// Runs the output item at the specified index.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of `OutputListBuilder`.
    /// - `idx`: The index of the output item to run.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current instance, allowing for method chaining.
    ///
    /// If the index is out of bounds, the list remains unchanged.
    pub fn run_idx(&mut self, idx: usize) -> &mut Self {
        if idx >= self.output_list.len() {
            return self;
        }
        let output: Output<'_> = self.query_idx(idx);
        output.output();
        self
    }
}
