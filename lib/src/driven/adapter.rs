use crate::core::Driven;
use crate::core::Plant;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::PoisonError;
use std::sync::RwLock;

/// Storage type the driven adapter operates on
pub type Storage = Arc<RwLock<HashMap<String, Plant>>>;

/// Driven adapter errors
#[derive(Debug)]
pub enum DrivenError {
    NotFoundError,
    WriteError,
}

impl Display for DrivenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFoundError => write!(f, "NotFoundError"),
            Self::WriteError => write!(f, "WriteError"),
        }
    }
}

impl<T> From<PoisonError<T>> for DrivenError {
    fn from(_value: PoisonError<T>) -> Self {
        DrivenError::WriteError
    }
}

impl From<DrivenError> for String {
    fn from(value: DrivenError) -> Self {
        value.to_string()
    }
}

/// Driven adapter
#[derive(Debug, Clone)]
pub struct DrivenAdapter {
    storage: Storage,
}

impl DrivenAdapter {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl Driven for DrivenAdapter {
    type Error = DrivenError;

    fn load(&self, id: Option<&str>) -> Result<Vec<Plant>, Self::Error> {
        let s = self.storage.read()?;

        match id {
            Some(id) => s
                .get(id)
                .map(|plant| vec![plant.clone()])
                .ok_or(DrivenError::NotFoundError),
            None => Ok(s.values().cloned().collect()),
        }
    }

    fn save(&self, plant: Plant) -> Result<Plant, Self::Error> {
        let mut s = self.storage.write()?;
        s.insert(plant.id().to_owned(), plant.to_owned());
        Ok(plant)
    }

    fn delete(&self, id: &str) -> Result<(), Self::Error> {
        let mut s = self.storage.write()?;
        s.remove(id);
        Ok(())
    }
}
