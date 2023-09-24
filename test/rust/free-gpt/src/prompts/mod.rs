pub mod user {

    // Adds the given prefix to the string and a space at the end
    pub fn greet(name: &str, content: &str) -> String {
        let mut message = "Hello chatGPT, i am ".to_owned();
        message.push_str(name);
        message.push_str(" ");
        message.push_str(content);
        return message;
    }
}
