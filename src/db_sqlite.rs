// pub mod db_sqlite {
//     extern crate rusqlite;
//
// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
//
// pub fn create_db_sqlite() -> Result<()> {
//     let conn = Connection::open("cats.db")?;
//
//     conn.execute(
//         "create table if not exists cat_colors (
//              id integer primary key,
//              name text not null unique
//          )",
//         NO_PARAMS,
//     )?;
//     conn.execute(
//         "create table if not exists cats (
//              id integer primary key,
//              name text not null,
//              color_id integer not null references cat_colors(id)
//          )",
//         NO_PARAMS,
//     )?;
//
//     Ok(())
// }
//     extern crate rusqlite;
//
// use rusqlite::NO_PARAMS;
// use rusqlite::{Connection, Result};
// use std::collections::HashMap;
//
// #[derive(Debug)]
// struct Cat {
//     name: String,
//     color: String,
// }
//
// fn main() -> Result<()> {
//     let conn = Connection::open("cats.db")?;
//
//     let mut cat_colors = HashMap::new();
//     cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
//     cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);
//
//     for (color, catnames) in &cat_colors {
//         conn.execute(
//             "INSERT INTO cat_colors (name) values (?1)",
//             &[&color.to_string()],
//         )?;
//         let last_id: String = conn.last_insert_rowid().to_string();
//
//         for cat in catnames {
//             conn.execute(
//                 "INSERT INTO cats (name, color_id) values (?1, ?2)",
//                 &[&cat.to_string(), &last_id],
//             )?;
//         }
//     }
//     let mut stmt = conn.prepare(
//         "SELECT c.name, cc.name from cats c
//          INNER JOIN cat_colors cc
//          ON cc.id = c.color_id;",
//     )?;
//
//     let cats = stmt.query_map(NO_PARAMS, |row| {
//         Ok(Cat {
//             name: row.get(0)?,
//             color: row.get(1)?,
//         })
//     })?;
//
//     for cat in cats {
//         println!("Found cat {:?}", cat);
//     }
//
//     Ok(())
// }
// }