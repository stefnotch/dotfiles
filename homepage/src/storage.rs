//! Code taken from https://github.com/DioxusLabs/sdk
//! and from https://dioxuslabs.com/learn/0.7/essentials/advanced/custom_hooks/
use dioxus::{
    hooks::{use_effect, use_signal},
    signals::{ReadableExt as _, Signal, WritableExt as _},
};
use serde::{Serialize, de::DeserializeOwned};

/// A persistent storage hook that can be used to store data across application reloads.
/// Uses local storage.
pub fn use_persistent<
    T: Serialize + DeserializeOwned + Clone + Send + Sync + PartialEq + 'static,
>(
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> UsePersistent<T> {
    let mut state = use_signal(move || {
        let key = key.to_string();
        let value = init(); // storage_get::<T>(&key).unwrap_or_else(|| init());
        StorageEntry { key, value }
    });
    use_effect(move || {
        state.with_mut(|entry| {
            if let Some(value) = storage_get::<T>(&entry.key) {
                entry.value = value;
            }
        });
    });
    UsePersistent { inner: state }
}

struct StorageEntry<T> {
    key: String,
    value: T,
}

/// Storage that persists across application reloads
pub struct UsePersistent<T: 'static> {
    inner: Signal<StorageEntry<T>>,
}

impl<T> Clone for UsePersistent<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UsePersistent<T> {}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UsePersistent<T> {
    /// Returns a reference to the value
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    /// Sets the value
    pub fn set(&mut self, value: T) {
        let mut inner = self.inner.write();
        storage_set(&inner.key, &value);
        inner.value = value;
    }
}

#[cfg(target_family = "wasm")]
fn storage_set<T: Serialize>(key: &str, value: &T) -> Option<()> {
    let as_str = serde_to_string(value);
    web_sys::window()?
        .local_storage()
        .ok()??
        .set_item(&key, &as_str)
        .ok()?;
    Some(())
}

#[cfg(not(target_family = "wasm"))]
fn storage_set<T: Serialize>(_key: &str, _value: &T) -> Option<()> {
    None
}

#[cfg(target_family = "wasm")]
fn storage_get<T: DeserializeOwned>(key: &str) -> Option<T> {
    let s: String = web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(key)
        .ok()??;
    try_serde_from_string(&s)
}

#[cfg(not(target_family = "wasm"))]
fn storage_get<T: DeserializeOwned>(_key: &str) -> Option<T> {
    None
}

/// Serializes a value to a string and compresses it.
pub(crate) fn serde_to_string<T: Serialize>(value: &T) -> String {
    let mut serialized = Vec::new();
    ciborium::into_writer(value, &mut serialized).unwrap();

    let as_str: String = serialized
        .iter()
        .flat_map(|u| {
            [
                char::from_digit(((*u & 0xF0) >> 4).into(), 16).unwrap(),
                char::from_digit((*u & 0x0F).into(), 16).unwrap(),
            ]
            .into_iter()
        })
        .collect();
    as_str
}

pub(crate) fn try_serde_from_string<T: DeserializeOwned>(value: &str) -> Option<T> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        let n1 = c.to_digit(16)?;
        let c2 = chars.next()?;
        let n2 = c2.to_digit(16)?;
        bytes.push((n1 * 16 + n2) as u8);
    }

    ciborium::from_reader(std::io::Cursor::new(bytes)).ok()
}
