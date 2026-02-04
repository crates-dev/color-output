use crate::*;

impl<'a> Default for ColorOutputList<'a> {
    /// Provides a default implementation for ColorOutputList.
    ///
    /// # Returns
    ///
    /// - `ColorOutputList` - New instance containing a single default ColorOutput
    #[inline(always)]
    fn default() -> Self {
        ColorOutputList(vec![ColorOutput::<'a>::default()])
    }
}

impl<'a> Deref for ColorOutputList<'a> {
    type Target = Vec<ColorOutput<'a>>;

    /// Dereferences ColorOutputList to its internal Vec of ColorOutputs.
    ///
    /// # Returns
    ///
    /// - `&Vec<ColorOutput>` - Reference to the internal vector of outputs
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> IntoIterator for &'a ColorOutputList<'a> {
    type Item = &'a ColorOutput<'a>;
    type IntoIter = Iter<'a, ColorOutput<'a>>;

    /// Returns an iterator over the elements of the internal Vec.
    ///
    /// # Returns
    ///
    /// - `Iter<ColorOutput>` - Iterator over references to ColorOutput elements
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> ColorOutputList<'a> {
    /// Provides an iterator over the elements in the internal `Vec<ColorOutput<'a>>`.
    ///
    /// # Returns
    /// - `Iter<'_, ColorOutput<'a>>`: An iterator over references to `ColorOutput` elements.
    #[inline(always)]
    pub fn iter(&self) -> std::slice::Iter<'_, ColorOutput<'a>> {
        self.0.iter()
    }

    /// ColorOutputs the content of each `ColorOutput` in the list.
    ///
    /// This method clones the `ColorOutputList` and iterates through its elements, calling the `output` method on each cloned `ColorOutput`.
    ///
    /// # Returns
    /// - `()` : Nothing is returned.
    pub fn output(self) {
        output_list(&self.to_vec());
    }
}
