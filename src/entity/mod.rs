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

impl Message {
    pub fn update(&mut self, value: impl Into<String>) {
        self.0 = value.into()
    }
}
