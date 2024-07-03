use crate::dto;
use crate::response::MyResponse;
use hexagonal::core::Driving;
use hexagonal::DrivenAdapter;
use hexagonal::DrivingAdapter;
use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;

pub(crate) struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/plants/:id", method = "get")]
    async fn read(&self, driven: Data<&DrivenAdapter>, id: Path<String>) -> MyResponse<dto::Plant> {
        let result = DrivingAdapter::read(*driven, &id);

        result.map(|plant| Some(plant.into())).into()
    }

    #[oai(path = "/plants", method = "get")]
    async fn read_collection(&self, driven: Data<&DrivenAdapter>) -> MyResponse<Vec<dto::Plant>> {
        let result = DrivingAdapter::read_collection(*driven);

        result
            .map(|plants| Some(plants.into_iter().map(|plant| (plant.into())).collect()))
            .into()
    }

    #[oai(path = "/plants", method = "post")]
    async fn create(
        &self,
        driven: Data<&DrivenAdapter>,
        plant: Json<dto::PlantNew>,
    ) -> MyResponse<dto::Plant> {
        let result = DrivingAdapter::create(*driven, &plant.name, &plant.leaf_type);

        result.map(|plant| Some(plant.into())).into()
    }

    #[oai(path = "/plants/:id", method = "put")]
    async fn update(
        &self,
        driven: Data<&DrivenAdapter>,
        changeset: Json<dto::PlantUpdate>,
        id: Path<String>,
    ) -> MyResponse<dto::Plant> {
        let result = DrivingAdapter::water_plant(*driven, &id, &changeset.watered_at);

        result.map(|plant| Some(plant.into())).into()
    }

    #[oai(path = "/plants/:id", method = "delete")]
    async fn delete(
        &self,
        driven: Data<&DrivenAdapter>,
        id: Path<String>,
    ) -> MyResponse<dto::Plant> {
        let result = DrivingAdapter::delete(*driven, &id);

        result.map(|_| None).into()
    }
}
