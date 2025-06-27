#[cfg(test)]
pub mod env {
    pub struct MockEnv {
        key: String,
        original: Option<String>,
    }

    impl MockEnv {
        pub fn set<K: Into<String>, V: Into<String>>(key: K, value: V) -> Self {
            let key = key.into();
            let original = std::env::var(&key).ok();
            std::env::set_var(&key, value.into());
            Self { key, original }
        }
    }

    impl Drop for MockEnv {
        fn drop(&mut self) {
            match &self.original {
                Some(val) => std::env::set_var(&self.key, val),
                None => std::env::remove_var(&self.key),
            }
        }
    }
}
