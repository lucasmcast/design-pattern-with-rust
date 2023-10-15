use crate::user::User;

pub struct UserCollection {
    users: Vec<User>
}

impl UserCollection {

    pub fn new() -> Self {
        Self {
            users: Vec::new()
        }
    }

    pub fn add(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn iter(&self) -> UserIterator {
        UserIterator { 
            index: 0, 
            user_collection: self 
        }
    }
}

pub struct UserIterator<'a>{
    index: usize,
    user_collection: &'a UserCollection,
}

impl<'a> Iterator for UserIterator<'a> {
    type Item = &'a User;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_collection.users.len() {
            let user = &self.user_collection.users[self.index];
            self.index += 1;
            return Some(user);
        }

        None    
    }
}



