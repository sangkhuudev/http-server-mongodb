use actix_web::{post, web, HttpResponse};
use crate::{models::dog_model::{Dog, DogRequest}, services::db::Database};

#[post("/dog")]
pub async fn create_dog(
    db: web::Data<Database>,
    request: web::Json<DogRequest>
) -> HttpResponse {
    match db
        .create_dog(
            Dog::try_from(DogRequest {
                owner: request.owner.clone(),
                name: request.name.clone(),
                age: request.age.clone(),
                breed: request.breed.clone(),
            })
            .expect("Error converting DogRequest to Dog."),
        )
        .await {
            Ok(booking) => HttpResponse::Ok().json(booking),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string())
        }
}