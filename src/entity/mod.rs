use crate::adapter::PrintMessage;

#[derive(Default)]
pub struct Message(String);

impl From<String> for Message {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl PrintMessage for Message {
    fn get_message(&self) -> String {
        self.0.clone()
    }
}
