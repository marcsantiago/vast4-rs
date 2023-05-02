#[derive(PartialEq, Debug)]
pub struct UnknownEvent {
    pub body: String,
}

impl Clone for UnknownEvent {
    fn clone(&self) -> UnknownEvent {
        UnknownEvent {
            body: self.body.clone(),
        }
    }
}
