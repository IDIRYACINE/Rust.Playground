use std::sync::Mutex;

static mut USERS: Vec<String> = Vec::new();

static USERS_MUTEX: Mutex<()> = Mutex::new(());

pub fn register_user(user: String) {
    let lock = USERS_MUTEX.lock().unwrap();

    unsafe {
        USERS.push(user);
    }

    std::mem::drop(lock);
}

pub fn get_users() -> Vec<String> {
    let lock = USERS_MUTEX.lock().unwrap();
    let users_clone: Vec<String>;

    unsafe {
        users_clone = USERS.clone();
    }
    std::mem::drop(lock);

    return users_clone;
}
