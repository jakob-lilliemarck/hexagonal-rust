use crate::core::CoreError;
use crate::core::Driven;
use crate::core::Driving;
use crate::core::Plant;
use chrono::DateTime;
use ulid::Ulid;

#[derive(Debug)]
pub enum DrivingError {
    DrivenError(String),
    NotFound,
    BadRequest,
}

impl<T> From<T> for DrivingError
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::DrivenError(value.into())
    }
}

impl From<CoreError> for DrivingError {
    fn from(value: CoreError) -> Self {
        match value {
            CoreError::InvariantError => Self::BadRequest,
        }
    }
}

pub struct DrivingAdapter;

impl Driving for DrivingAdapter {
    type Error = DrivingError;

    fn read(driven: &impl Driven, id: &str) -> Result<Plant, Self::Error> {
        let results = driven.load(Some(id))?;
        let plant = results.first().ok_or(DrivingError::NotFound)?;
        Ok(plant.to_owned())
    }

    fn read_collection(driven: &impl Driven) -> Result<Vec<Plant>, Self::Error> {
        let plants = driven.load(None)?;
        Ok(plants)
    }

    fn create(driven: &impl Driven, name: &str, leaf_type: &str) -> Result<Plant, Self::Error> {
        let id = Ulid::new();
        let plant = Plant::new(id.to_string(), name.to_owned(), leaf_type)?;
        let plant = driven.save(plant.clone())?;
        Ok(plant)
    }

    fn water_plant(driven: &impl Driven, id: &str, watered_at: &str) -> Result<Plant, Self::Error> {
        let plant = DrivingAdapter::read(driven, id)?;
        let watered_at = DateTime::parse_from_rfc3339(watered_at)
            .map(|datetime| datetime.to_utc())
            .map_err(|_| CoreError::InvariantError)?;
        let plant = plant.watered_at(watered_at)?;
        let plant = driven.save(plant)?;
        Ok(plant)
    }

    fn delete(driven: &impl Driven, id: &str) -> Result<(), Self::Error> {
        driven.delete(id)?;
        Ok(())
    }
}
