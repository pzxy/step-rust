use std::env;

use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;

// 详细教程 https://github.com/launchbadge/sqlx/blob/main/examples
// cargo install sqlx-cli
// sqlx db create ,会创建一个数据库，这个参数和.env文件中的数据库名有关,也就是环境变量中的值。
// sqlx migrate run , 执行migrations目录下.sql文件不能和数据库名一样。打印类似 Applied 20221113/migrate todos (282.75µs) 才算执行成功。
// 生成todos.db后，复制到项目根目录一份。如果就在根目录就不用动了。
// cargo run --bin sqlite
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let pool = SqlitePool::connect(&database_url).await?;
    // 插入数据
    let description = String::from("todo description");
    let todo_id = add_todo(&pool, description).await?;
    println!("Added new todo with id {}", todo_id);

    // 更新数据 id
    let id = 1;
    if complete_todo(&pool, id).await? {
        println!("Todo {} is marked as done", id);
    } else {
        println!("Invalid id {}", id);
    }
    // 打印数据
    println!("Printing list of all todos");
    list_todos(&pool).await?;
    Ok(())
}

async fn add_todo(pool: &SqlitePool, description: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
        description
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = ?1
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

async fn list_todos(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.done { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }

    Ok(())
}
