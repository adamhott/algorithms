// LeetCode 981. Time Based Key-Value Store

//Design a time-based key-value data structure that
//can store multiple values for the same key at different 
//time stamps and retrieve the key's value at a certain 
//timestamp.

//Implement the TimeMap class:

//TimeMap() Initializes the object of the data structure.

//void set(String key, String value, int timestamp) 
//Stores the key key with the value value at the given time timestamp.

//String get(String key, int timestamp)
// Returns a value such that set was called previously, with timestamp_prev <= timestamp. If there are multiple such values, it returns the value associated with the largest timestamp_prev. If there are no values, it returns "".

use std::collections::HashMap;

struct TimeMap {
    key_store: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            key_store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.key_store
            .entry(key)
            .or_insert_with(Vec::new)
            .push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.key_store.get(&key) {
            let mut l = 0;
            let mut r = values.len() as i32 - 1;
            let mut res = String::new();

            while l <= r {
                let m = (l + r) / 2;
                if values[m as usize].1 <= timestamp {
                    res = values[m as usize].0.clone();
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }

            res
        } else {
            String::new()
        }
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_map() {
        let mut time_map = TimeMap::new();

        // Corresponds to timeMap.set("foo", "bar", 1);
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(time_map.get("foo".to_string(), 1), "bar");
        assert_eq!(time_map.get("foo".to_string(), 3), "bar");

        // Corresponds to timeMap.set("foo", "bar2", 4);
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2");
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2");
    }
}
