pub mod tuples {
    pub fn tuples() {
        let tuple_0 = (String::from("Denis"), 11);
        let (name, grade) = tuple_0;

        println!("Имя: {}, балл {}", name, grade);
    }
}
