use sqlx::{mysql::MySqlPool, Error, FromRow};

#[derive(Clone, Debug, FromRow)]
pub struct RoomData {
    pub id: i32,
    owner: i32,
    name: String,
    description: String,
    password: String,
    max_users: i32,
    floor_plan: String,
    door_x: i32,
    door_y: i32,
    walk_through: bool,
    floor_thickness: i32,
    wall_thickness: i32,
    wall_height: i32,
    wall_hidden: bool,
}

impl RoomData {
    pub async fn by_id(id: i32, conn: MySqlPool) -> Result<RoomData, Error> {
        let row = sqlx::query("SELECT * FROM rooms WHERE id = ?")
            .bind(id)
            .fetch_one(&conn).await?;
            

        RoomData::from_row(&row)
    }
}
