/// View that loads and commits its state to a model.
pub trait View {
    /// Load widget states from the project model.
    fn load_state(&self);

    /// Commit widget states to the project model.
    fn commit_state(&self);
}
