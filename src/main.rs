// extern crate rusqlite;

extern crate core;
extern crate ansi_term;

use crate::arithmetic::arith::arith;
use crate::basic::basic::basic_func;
use crate::cicle::cicle::{cicle_for, cicle_loop, cicle_while};
use crate::color_term::color_term::color_term;
use crate::diskremenant::diskremenant::diskrem;
use crate::format_out::fmt_out::format_out;
use crate::io_put::io_put::io_put;
use crate::massiv::mod_massiv::fn_massiv;
use crate::operator::operator::operator;
use crate::r#match::matc::matc;
use crate::structure::structure::hyp;
use crate::tuples::tuples::tuples;
use crate::types::types::{cast, fn_string, inference, literals, types};
use crate::variable::variable::{area_of_visibility, pre_announcement, variability, variable};
// use crate::create_sqlite::create_sqlite::create_table;
// use crate::read_sqlite::read_db::read_sqlite;
use crate::db_postgres::db_postgres::{create_table, read_err_all, read_err_code};

use ansi_term::Colour;
use ansi_term::Style;
mod arithmetic;
mod basic;
mod cicle;
mod color_term;
mod create_sqlite;
mod db_postgres;
mod diskremenant;
mod format_out;
mod io_put;
mod massiv;
mod r#match;
mod operator;
mod structure;
mod tuples;
mod types;
mod variable;
// mod create_sqlite;
// mod read_sqlite;

#[derive(Debug)]

struct User {
    actives: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn build_user(username: String, email: String) -> User {
    User {
        actives: true,
        username,
        email,
        sign_in_count: 1,
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user_1 = User {
        actives: true,
        username: String::from("someuser123"),
        email: String::from("someuser@gmail.com"),
        sign_in_count: 1,
    };
    // Чтобы получить конкретное значение из структуры, мы используем запись через точку.
    // Например, чтобы получить доступ к адресу электронной почты этого пользователя,
    // мы используем user1.email.
    println!("user name: {}", Colour::Purple.paint(&user_1.username));
    // Если экземпляр является изменяемым, мы можем поменять значение, используя точечную нотацию
    // и присвоение к конкретному полю. В Листинге 5-3 показано, как изменить значение в поле
    // email изменяемого экземпляра User.
    println!("email до изменения: {}", Colour::Blue.paint(&user_1.email));
    user_1.email = String::from("anotheremail@example.com");
    println!("email после изменения: {}", Colour::Purple.paint(&user_1.email));
    let user_2 = build_user("user2".to_string(), "user2@example.com".to_string());
    println!("user 2: {:?}", user_2.email);

    let rect_1 = Rectangle {
        width: 32,
        height: 47,
    };
    println!("площадь прямоугольника равна: {}", rect_1.area());

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    basic_func();
    format_out();
    operator();
    variable();
    variability();
    area_of_visibility();
    pre_announcement();
    types();
    arith();
    inference();
    literals();
    cast();
    cicle_loop(3);
    cicle_while(3);
    cicle_for();
    matc(4);
    io_put();
    diskrem();
    fn_massiv();
    tuples();
    hyp();
    fn_string();
    color_term();
    // create_table();
    // read_sqlite();

    println!("{:?}", create_table().expect("не удалось подключится к БД"));
    println!("{:?}", read_err_all().expect("не удалось подключится к БД"));

    let errdesc = read_err_code(56).unwrap();
    println!("{}", Colour::RGB(190, 70, 15).paint(&errdesc));

    // для записи в БД используется код ниже
    // let mut insert_row = String::new();
    // insert_row = "тестовая запись 485".to_string();
    // write_err_code(485, insert_row);

    // println!("{:?}", read_err_code(484));
}
