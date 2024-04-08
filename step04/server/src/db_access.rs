use crate::models::*;
use chrono::NaiveDateTime;
use sqlx::PgPool;

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Vec<Course> {
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, name, time
        FROM course
        WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    rows.iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            name: r.name.clone(),
            time: Some(NaiveDateTime::from(r.time.unwrap())),
        })
        .collect()
}
