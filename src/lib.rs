// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-crun: crun component model.
//!
//! Real implementation: crun_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrunItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrunError {
    EmptyCrunItemName,
    DuplicateCrunItem(String),
    EmptyEventSubject,
}

impl fmt::Display for CrunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCrunItemName => write!(f, "crun_item name must not be empty"),
            Self::DuplicateCrunItem(id) => write!(f, "duplicate crun_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for CrunError {}

pub struct Crun {
    pub name: String,
    crun_items: BTreeMap<String, CrunItem>,
    events: BTreeMap<String, Event>,
}

impl Crun {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            crun_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_crun_item(&mut self, item: CrunItem) -> Result<(), CrunError> {
        if item.name.trim().is_empty() {
            return Err(CrunError::EmptyCrunItemName);
        }
        let id = item.id.clone();
        if self.crun_items.contains_key(&id) {
            return Err(CrunError::DuplicateCrunItem(id.clone()));
        }
        self.crun_items.insert(id, item);
        Ok(())
    }

    pub fn crun_item_count(&self) -> usize {
        self.crun_items.len()
    }

    pub fn get_crun_item(&self, id: &str) -> Option<&CrunItem> {
        self.crun_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), CrunError> {
        if event.subject.trim().is_empty() {
            return Err(CrunError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_crun_item(id: &str, name: &str) -> CrunItem {
        CrunItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Crun::new("Crun");
        assert_eq!(m.name, "Crun");
        assert_eq!(m.crun_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_crun_item() {
        let mut m = Crun::new("Crun");
        m.add_crun_item(sample_crun_item("a1", "Alpha")).unwrap();
        assert_eq!(m.crun_item_count(), 1);
        assert!(m.get_crun_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Crun::new("Crun");
        assert!(m.add_crun_item(sample_crun_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Crun::new("Crun");
        m.add_crun_item(sample_crun_item("x", "One")).unwrap();
        assert!(m.add_crun_item(sample_crun_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Crun::new("Crun");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
