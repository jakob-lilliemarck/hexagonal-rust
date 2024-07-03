use poem_openapi::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Object, Serialize, Deserialize, Clone)]
pub struct Plant {
    pub id: String,
    pub name: String,
    pub leaf_type: String,
    pub watered_at: Option<String>,
}

impl From<hexagonal::core::Plant> for Plant {
    fn from(value: hexagonal::core::Plant) -> Self {
        Self {
            id: value.id,
            name: value.name,
            leaf_type: value.leaf_type.to_string(),
            watered_at: value.watered_at.map(|datetime| datetime.to_rfc3339()),
        }
    }
}

#[derive(Debug, Object, Serialize, Deserialize, Clone)]
pub struct PlantNew {
    pub name: String,
    pub leaf_type: String,
}

#[derive(Debug, Object, Serialize, Deserialize, Clone)]
pub struct PlantUpdate {
    pub watered_at: String,
}
