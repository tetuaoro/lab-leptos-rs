pub trait PrintMessage: Default {
    fn get_message(&self) -> String;
}
