struct Temperature {
    degree_f : f64,
}



impl Temperature {
    fn freezing() -> Self {
        Self {
            degree_f : 32
        }
    }
    fn show_temp(&self) {
        println!("{:?} degree F" , self.degree_f);
    }
}


fn main() {
    let hot = Temperature {
        degree_f : 99.9
    };


    hot.show_temp();
    // Temperature::show_temp(hot);
    let cold = Temperature::freezing();
     cold.show_temp();
}