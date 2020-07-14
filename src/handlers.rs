use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{ web, Error, HttpResponse };
use serde::{ Deserialize, Serialize };
use std::vec::vec;
use actix_web::error::PayloadError::Http2Payload;
use crate::models::Users;

//type
#[derive(Deserialize, Deserialize, Debug)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

//GET /users
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn);
    OK(items)
}

fn add_single_ser

//GET /users/{id}
pub async fn get_user_by_id( db: web::Data<Pool>, user_id: web::Path<i32> ) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?

    )
}

//POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_))
}





