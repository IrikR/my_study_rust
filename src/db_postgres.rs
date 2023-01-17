extern crate postgres;

pub mod db_postgres {
    use postgres::{Client, Error, NoTls, Row};

    struct ErrDescription {
        id_err: i32,
        err_desc: String,
    }

    pub fn create_table() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://osiris:174785266@localhost/template1", NoTls)?;

        client.batch_execute(
            "
         CREATE TABLE IF NOT EXISTS author (
             id              SERIAL PRIMARY KEY,
             name            VARCHAR NOT NULL,
             country         VARCHAR NOT NULL
             )
     ",
        )?;

        client.batch_execute(
            "
         CREATE TABLE IF NOT EXISTS book  (
             id              SERIAL PRIMARY KEY,
             title           VARCHAR NOT NULL,
             author_id       INTEGER NOT NULL REFERENCES author
             )
     ",
        )?;

        Ok(())
    }

    // считывание всей таблицы
    pub fn read_err_all() -> Result<(), Error> {
        let mut client = Client::connect(
            "postgresql://postgres:174785266@localhost/test_postgres",
            NoTls,
        )?;

        for row in client.query("SELECT id_err, err_desc FROM err_code", &[])? {
            let err_desc = ErrDescription {
                id_err: row.get(0),
                err_desc: row.get(1),
            };
            println!(
                "Err code: {} description: {}",
                err_desc.id_err, err_desc.err_desc
            );
        }

        Ok(())
    }

    // функция считывания из БД по id
    pub fn read_err_code(id_err: i32) -> Result<String, Error> {
        let select_id = format!(
            "SELECT id_err, err_desc FROM err_code where id_err={};",
            id_err
        );
        let mut client = Client::connect(
            "postgresql://postgres:174785266@localhost/test_postgres",
            NoTls,
        )
        .expect("не удалось подключится к базе данных");
        let err_desc = client
            .query_one(&select_id, &[])
            .expect("не удалось считать строку данных !!!");
        // println!("{:?}", &err_desc);
        let row = ErrDescription {
            id_err: err_desc.get(0),
            err_desc: err_desc.get(1),
        };
        let desc = row.err_desc;
        Ok(desc)
    }

    // функция записи в БД
    pub fn write_err_code(id_err: i32, err_desc: String) {
        let row = ErrDescription { id_err, err_desc };
        let mut client = Client::connect(
            "postgresql://postgres:174785266@localhost/test_postgres",
            NoTls,
        )
        .expect("не удалось подключится к базе данных");

        client
            .execute(
                "insert into err_code (id_err, err_desc) values ($1, $2)",
                &[&row.id_err, &row.err_desc],
            )
            .expect("не удалось записать данные");
    }
}

// CREATE TABLE products (id INT, name TEXT, quantity INT)
// INSERT INTO products (id, name, quantity) VALUES (1, 'first product', 20);
// SELECT id, name, quantity FROM products;
// SELECT * FROM products;
//  copy err_code from 'D:\config\config_stend\to_postgres_v4.csv' delimiter ';' csv header
// \COPY err_code from 'D:\config\config_stend\to_postgres_v0.txt' delimiter ';' csv header;
