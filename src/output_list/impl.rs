use crate::*;

impl<'a> Default for OutputList<'a> {
    /// Provides a default implementation for OutputList.
    ///
    /// # Returns
    ///
    /// - `OutputList` - New instance containing a single default Output
    #[inline(always)]
    fn default() -> Self {
        OutputList(vec![Output::<'a>::default()])
    }
}

impl<'a> Deref for OutputList<'a> {
    type Target = Vec<Output<'a>>;

    /// Dereferences OutputList to its internal Vec of Outputs.
    ///
    /// # Returns
    ///
    /// - `&Vec<Output>` - Reference to the internal vector of outputs
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> IntoIterator for &'a OutputList<'a> {
    type Item = &'a Output<'a>;
    type IntoIter = Iter<'a, Output<'a>>;

    /// Returns an iterator over the elements of the internal Vec.
    ///
    /// # Returns
    ///
    /// - `Iter<Output>` - Iterator over references to Output elements
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> OutputList<'a> {
    /// Provides an iterator over the elements in the internal `Vec<Output<'a>>`.
    ///
    /// # Returns
    /// - `Iter<'_, Output<'a>>`: An iterator over references to `Output` elements.
    #[inline(always)]
    pub fn iter(&self) -> std::slice::Iter<'_, Output<'a>> {
        self.0.iter()
    }

    /// Outputs the content of each `Output` in the list.
    ///
    /// This method clones the `OutputList` and iterates through its elements, calling the `output` method on each cloned `Output`.
    ///
    /// # Returns
    /// - `()` : Nothing is returned.
    pub fn output(self) {
        output_list(&self.to_vec());
    }
}
