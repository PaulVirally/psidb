pub mod data;
pub mod transform;
pub mod connection;
pub mod action;

pub trait Entry {
    fn get_id(&self) -> u64;
}

pub fn entry_in<T: PartialEq + Entry>(entry: &T, vec: &Vec<T>) -> (bool, u64) {
    let mut exists = false;
    let mut id = 0;

    for other_entry in vec {
        if entry == other_entry {
            exists = true;
            id = other_entry.get_id();
            break;
        }
    }
    (exists, id)
}

pub fn id_in<T: Entry>(id: u64, vec: &Vec<T>) -> bool {
    vec.iter().any(|entry| entry.get_id() == id)
}
