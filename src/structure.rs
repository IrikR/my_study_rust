pub mod structure {
    struct Triangle0 {
        cat1: f64,
        cat2: f64,
    }

    fn find_hyp(c1: f64, c2: f64) -> f64 {
        (c1 * c1 + c2 * c2).sqrt()
    }

    pub fn hyp() {
        let tr1 = Triangle0 {
            cat1: 3.0,
            cat2: 4.0,
        };
        let hyp = find_hyp(tr1.cat1, tr1.cat2);
        println!("гипотенуза равна: {}", hyp);
    }

    // impl Triangle1 {
    //     fn find_hyp(&self) -> f64 {
    //         (self.cat1 * self.cat1 + self.cat2 * self.cat2).sqrt()
    //     }
    // }
    //
    // pub fn hyp_2 (){
    //     let tr2 = Triangle1{
    //     cat1: 3.0,
    //     cat2: 4.0
    //     };
    //     let hyp = tr2.find_hyp();
    // }
}
