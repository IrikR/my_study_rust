pub mod diskremenant {
    use std::io;

    pub fn diskrem() {
        println!("Решение квадратного уравнения");
        let mut a_str = String::new();
        let mut b_str = String::new();
        let mut c_str = String::new();

        println!("ввдете аргумент а");
        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {}
            Err(e) => {
                println!("Ошибка. {}", e);
            }
        }

        println!("ввдете аргумент b");
        match io::stdin().read_line(&mut b_str) {
            Ok(_) => {}
            Err(e) => {
                println!("Ошибка. {}", e);
            }
        }

        println!("ввдете аргумент c");
        match io::stdin().read_line(&mut c_str) {
            Ok(_) => {}
            Err(e) => {
                println!("Ошибка. {}", e);
            }
        }

        let a_fl: f64 = a_str.trim().parse().unwrap();
        let b_fl: f64 = b_str.trim().parse().unwrap();
        let c_fl: f64 = c_str.trim().parse().unwrap();
        let d: f64 = (b_fl * b_fl) - (4.0 * a_fl * c_fl);

        if d > 0.0 {
            let x1 = ((-b_fl) + d.sqrt()) / (2.0 * a_fl);
            let x2 = ((-b_fl) - d.sqrt()) / (2.0 * a_fl);
            println!(
                "{}x^2 + {}x + {} = 0, корни уравнения x1 = {}, x2 = {}",
                a_fl, b_fl, c_fl, x1, x2
            );
        }

        if d == 0.0 {
            let x1 = (-b_fl) / (2.0 * a_fl);
            println!(
                "{}x^2 + {}x + {} = 0, корень уравнения x1 = {}",
                a_fl, b_fl, c_fl, x1
            );
        }

        if d < 0.0 {
            println!(
                "{}x^2 + {}x + {} = 0, корней НЕТ, D = {}",
                a_fl, b_fl, c_fl, d
            );
        }

        // println!("{}x^2 + {}x + {} = 0", a_fl, b_fl, c_fl);
    }
}
