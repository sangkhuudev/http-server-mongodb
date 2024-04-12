use actix_web::{get, post, put, web::{self, Path}, HttpResponse};
use crate::{models::booking_model::{Booking, BookingRequest}, services::db::Database};

#[post("/booking")]
pub async fn create_booking(
    db: web::Data<Database>,
    request: web::Json<BookingRequest>
) -> HttpResponse {
    match db
        .create_booking(
            Booking::try_from(BookingRequest {
                owner: request.owner.clone(),
                start_time: request.start_time.clone(),
                duration_in_minutes: request.duration_in_minutes.clone(),

            })
            .expect("Error converting ownerRequest to Owner."),
        )
        .await {
            Ok(booking) => HttpResponse::Ok().json(booking),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string())
        }
}

#[get("/bookings")]
pub async fn get_bookings(db: web::Data<Database>) -> HttpResponse {
    match db.get_bookings().await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[put("/booking/{id}/cancle")]
pub async fn cancle_booking(db: web::Data<Database>, path: Path<(String,)>) -> HttpResponse {
    let id = path.into_inner().0;
    match db.cancle_booking(id.as_str()).await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}