pub mod user {

    use rocket::serde::json::serde_json;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;

    use serde::{Deserialize, Serialize};
    use std::io::{Read, Write};
    use std::path::Path;

    use rand::random;
    use std::borrow::Borrow;
    use std::convert::TryFrom;
    use std::time::SystemTime;

    const FILE_EXTENSION: &str = "json";
    const FILE_STORAGE: &str = "file-storage";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct History {
        pub message: String,
        pub response: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Conversation {
        id: u8,
        title: String,
        index: usize,
        requests: i32,
        pub messages: Vec<History>,
        active: SystemTime,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        conversations: Vec<Conversation>,
    }

    pub fn get(user: &str, description: String, content: String) -> Conversation {
        let path: String = FILE_STORAGE.to_owned() + "/" + user + "." + FILE_EXTENSION;
        // Create a directory to the desired file
        let directory = Path::new(&path);

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&directory) {
            Err(why) => {
                warn!("could not open file for {} : {:?}", user, why);
                fs::create_dir_all(FILE_STORAGE).unwrap();
                File::create(&directory).unwrap()
            }
            Ok(file) => file,
        };

        // Read the file contents into a string
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => {
                warn!("could not read file for {} : {:?}", user, why);
            }
            Ok(s) => (),
        }

        let mut user_details: User = match serde_json::from_str(&s) {
            Err(why) => {
                // NOTE !!! In case of schema change, file contents will be overwritten !!!
                warn!("could not decode json for {} : {:?}", user, why);
                User {
                    conversations: vec![],
                }
            }
            Ok(User) => User,
        };

        let mut conversation = Conversation {
            id: random(),
            title: description.clone(),
            index: 0,
            requests: 0,
            messages: Vec::new(),
            active: SystemTime::now(),
        };

        let mut i: usize = 0;
        for conv in user_details.conversations.iter() {
            if conv.title.eq(&description) {
                // overwrite with saved details
                conversation.id = conv.id;
                conversation.requests = conv.requests;
                conversation.messages = conv.messages.clone();
                conversation.index = i.clone();
                break;
            }
            i = i + 1;
        }

        return conversation;
    }

    pub fn put(user: &str, limit: usize, mut conversation: Conversation, response: History) {
        let mut user_details = get_user(user);

        conversation.messages = [conversation.messages, [response].to_vec()].concat();

        if conversation.messages.len() > limit {
            conversation.messages.drain(0..1);
        }

        if conversation.requests == 0 {
            // if it s a new conversation
            conversation.requests = conversation.requests + 1;
            user_details.conversations =
                [user_details.conversations, [conversation].to_vec()].concat();
        } else {
            // otherwise lets put it back into the same spot
            conversation.requests = conversation.requests + 1;
            std::mem::replace(
                &mut user_details.conversations[conversation.index],
                conversation.clone(),
            );
        }

        // save back
        let file_name: String = FILE_STORAGE.to_owned() + "/" + user + "." + FILE_EXTENSION;
        // Create a directory to the desired file
        let path = Path::new(&file_name);
        let b = serde_json::to_string_pretty(&user_details).unwrap();
        let mut f = File::create(&path).unwrap();
        f.write_all(b.as_bytes()).expect("Unable to write data");
    }

    fn get_user(user: &str) -> User {
        let file_name: String = FILE_STORAGE.to_owned() + "/" + user + "." + FILE_EXTENSION;
        // Create a directory to the desired file
        let path = Path::new(&file_name);

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => {
                warn!("could not open file for {} : {:?}", user, why);
                fs::create_dir_all(FILE_STORAGE).unwrap();
                File::create(&path).unwrap()
            }
            Ok(file) => file,
        };

        // Read the file contents into a string
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => {
                warn!("could not read file for {} : {:?}", user, why);
            }
            Ok(s) => (),
        }

        let mut user_details: User = match serde_json::from_str(&s) {
            Err(why) => {
                warn!("could not decode json for {} : {:?}", user, why);
                User {
                    conversations: vec![],
                }
            }
            Ok(User) => User,
        };

        return user_details;
    }
}
