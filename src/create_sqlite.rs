// extern crate sqlite;
//
// // use crate::sqlite::
// pub mod create_sqlite {
//     pub fn create_table(){
//         let connection = sqlite::open(":memory:").unwrap();
//
//         let query = "
//             CREATE TABLE users (name TEXT, age INTEGER);
//             INSERT INTO users VALUES ('Alice', 42);
//             INSERT INTO users VALUES ('Bob', 69);
//         ";
//         connection.execute(query).unwrap();
//     }
// }
