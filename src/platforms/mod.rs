pub struct Platform {
    current_model: String,
    models: Vec<String>,
}

impl Platform {
    fn chat_and_generate(self: &Self, message: String) -> String {
        let _ = self.generate();
    }

    fn generate();
}
