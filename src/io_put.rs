pub mod io_put {
    use std::io;

    pub fn io_put() {
        let mut name = String::new();
        println!("Ваше имя?");
        match io::stdin().read_line(&mut name) {
            Ok(_) => {
                println!("Здравствуйте, {}", name);
            }
            Err(e) => {
                println!("ошибка {}", e);
            }
        }
    }
}
