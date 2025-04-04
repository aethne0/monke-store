#![allow(dead_code)] // TODO: remove
#![allow(unused_variables)] // TODO: remove

mod db;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::db::Db;

    #[test]
    fn test_get_set_get() {
        let mut db = Db::new();

        assert_eq!(db.get(&"key1".to_string()), None, "should not exist yet.");
        assert_eq!(db.set(&"key1".to_string(), &"val1".to_string()), Ok(()));
        assert!(db.get(&"key1".to_string()) == Some(&"val1".to_string()));
    }

    #[test]
    fn test_set4_get4() {
        let mut db = Db::new();
        assert_eq!(db.set(&"key1".to_string(), &"val1".to_string()), Ok(()));
        assert_eq!(db.set(&"key2".to_string(), &"val2".to_string()), Ok(()));
        assert_eq!(db.set(&"key3".to_string(), &"val3".to_string()), Ok(()));
        assert_eq!(db.set(&"key4".to_string(), &"val4".to_string()), Ok(()));
        assert!(db.get(&"key1".to_string()) == Some(&"val1".to_string()));
        assert!(db.get(&"key2".to_string()) == Some(&"val2".to_string()));
        assert!(db.get(&"key3".to_string()) == Some(&"val3".to_string()));
        assert!(db.get(&"key4".to_string()) == Some(&"val4".to_string()));
    }

    #[test]
    fn test_overwriting() {
        let mut db = Db::new();
        assert_eq!(db.set(&"key1".to_string(), &"val1".to_string()), Ok(()));
        assert_eq!(db.set(&"key1".to_string(), &"val2".to_string()), Ok(()));
        assert_eq!(db.set(&"key2".to_string(), &"val3".to_string()), Ok(()));
        assert_eq!(db.set(&"key2".to_string(), &"val4".to_string()), Ok(()));
        assert!(db.get(&"key1".to_string()) == Some(&"val2".to_string()));
        assert!(db.get(&"key2".to_string()) == Some(&"val4".to_string()));
    }
}
