// циклы
// loop - бесконечный цикл
// while
// for
pub mod cicle {
    pub fn cicle_loop(arg_num: i32) {
        let mut num = 0;
        loop {
            print!("num: {}\t", num);
            num += 1;
            if num == arg_num {
                break;
            }
        }
        println!();
    }
    pub fn cicle_while(arg_num: i32) {
        let mut num = 0;
        while num <= arg_num {
            print!("num {}\t", num);
            num += 1;
        }
        println!();
    }
    pub fn cicle_for() {
        for i in 0..11 {
            print!("{}\t", i);
        }
        println!();
    }
}
