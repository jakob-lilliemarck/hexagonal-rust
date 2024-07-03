use chrono::prelude::*;
use std::fmt::Debug;
use std::fmt::Display;

pub enum CoreError {
    InvariantError,
}

#[derive(Debug, Clone)]
pub enum LeafType {
    Needle,
    Sheath,
    Frond,
}

impl Display for LeafType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Needle => write!(f, "needle"),
            Self::Sheath => write!(f, "sheath"),
            Self::Frond => write!(f, "frond"),
        }
    }
}

impl TryFrom<&str> for LeafType {
    type Error = CoreError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "needle" => Ok(LeafType::Needle),
            "sheath" => Ok(LeafType::Sheath),
            "frond" => Ok(LeafType::Frond),
            _ => Err(CoreError::InvariantError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plant {
    pub id: String,
    pub name: String,
    pub leaf_type: LeafType,
    pub watered_at: Option<DateTime<Utc>>,
}

impl Plant {
    pub fn new(id: String, name: String, leaf_type: &str) -> Result<Self, CoreError> {
        let leaf_type = LeafType::try_from(leaf_type)?;

        Ok(Plant {
            id: id,
            name,
            leaf_type,
            watered_at: None,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn watered_at(mut self, watered_at: DateTime<Utc>) -> Result<Self, CoreError> {
        self.watered_at = Some(watered_at);
        Ok(self)
    }
}

pub trait Driving {
    type Error: Debug;

    fn read(driven: &impl Driven, id: &str) -> Result<Plant, Self::Error>;
    fn read_collection(driven: &impl Driven) -> Result<Vec<Plant>, Self::Error>;
    fn create(driven: &impl Driven, name: &str, leaf_type: &str) -> Result<Plant, Self::Error>;
    fn water_plant(driven: &impl Driven, id: &str, watered_at: &str) -> Result<Plant, Self::Error>;
    fn delete(driven: &impl Driven, id: &str) -> Result<(), Self::Error>;
}

pub trait Driven {
    type Error: Debug + Into<String>;

    fn load(&self, id: Option<&str>) -> Result<Vec<Plant>, Self::Error>;
    fn save(&self, plant: Plant) -> Result<Plant, Self::Error>;
    fn delete(&self, id: &str) -> Result<(), Self::Error>;
}
