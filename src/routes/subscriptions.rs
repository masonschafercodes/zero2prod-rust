use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct SubscriptionFormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<SubscriptionFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let query = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match query {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}
