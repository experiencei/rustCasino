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


//new line macro
struct Temperature {
    degree_f : i32,
}

impl Temperature { 
    
    fn boiling() -> Self {
        Self {
            degree_f : 568,
        }
    }


    fn show_Temperature(&self)  {
        println!(" degree in celsius is {:?}" , self.degree_f);
     }

    
}



fn main() {
  
 let hot = Temperature { degree_f : 456 }
 hot.show_Temperature();
 
 let cold = Temperature::boiling();
 cold.show_Temperature();
}


