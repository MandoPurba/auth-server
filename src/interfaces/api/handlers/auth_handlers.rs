use actix_web::{post, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::{
    application::{dtos::user_dtos::CreateUserDto, use_cases::register_user::RegisterUserUseCase},
    infrastructures::repositories::postgres_user_repository::PgUserRepository,
};

#[post("/register")]
async fn register_user(
    pool: web::Data<Pool<Postgres>>,
    form: web::Json<CreateUserDto>,
) -> impl Responder {
    // Create a repository instance
    let user_repository = PgUserRepository {
        pool: pool.get_ref().clone(),
    };

    // Create the use case instance
    let use_case = RegisterUserUseCase {
        repository: user_repository,
    };

    // Execute the use case
    match use_case.execute(form.into_inner()).await {
        Ok(user_dto) => HttpResponse::Created().json(user_dto),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}
