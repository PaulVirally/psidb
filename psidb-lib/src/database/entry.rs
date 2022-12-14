pub mod data;
pub mod transform;
pub mod connection;
pub mod action;

pub trait Entry {
    fn get_id(&self) -> u64;
}

pub fn entry_in<T: PartialEq + Entry>(entry: &T, arr: &[T]) -> (bool, u64) {
    let mut exists = false;
    let mut id = 0;

    for other_entry in arr {
        if entry == other_entry {
            exists = true;
            id = other_entry.get_id();
            break;
        }
    }
    (exists, id)
}

pub fn id_in<T: Entry>(id: u64, arr: &[T]) -> bool {
    arr.iter().any(|entry| entry.get_id() == id)
}
