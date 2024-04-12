use actix_web::{post, web, HttpResponse};
use crate::{models::owner_model::{Owner, OwnerRequest}, services::db::Database};

#[post("/owner")]
pub async fn create_owner(
    db: web::Data<Database>,
    request: web::Json<OwnerRequest>
) -> HttpResponse {
    match db
        .create_owner(
            Owner::try_from(OwnerRequest {
                name: request.name.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                address: request.address.clone(),

            })
            .expect("Error converting ownerRequest to Owner."),
        )
        .await {
            Ok(booking) => HttpResponse::Ok().json(booking),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string())
        }
}