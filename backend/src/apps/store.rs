pub struct Store<T, A> {
    pub state: T,
    reducer: Box<dyn Fn(T, A) -> T + Send>,
}

impl<T, A> Store<T, A>
where
    T: Copy + Clone + Default + PartialEq,
{
    pub fn new(reducer: Box<dyn Fn(T, A) -> T + Send>) -> Self {
        Self {
            state: T::default(),
            reducer,
        }
    }

    pub fn dispatch(&mut self, action: A) {
        self.state = (self.reducer)(self.state, action);
    }
}
