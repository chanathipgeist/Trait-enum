struct Classico { 
    year: &'static str,
    
}

trait Music {
    fn new(year: &'static str) -> Self;
    fn Gen(&self) -> &'static str;
    fn Tempo(&self) -> &'static str;
}


impl Music for Classico  {
    
    fn new(year: &'static str) -> Classico {
        Classico { year: year }
    }

    fn Gen(&self) -> &'static str {
        "Classical"
    }
    fn Tempo(&self) -> &'static str {
        "Sonata"
    }
     
}
fn main() {
    // Type annotation is necessary in this case.
    let mut Mu1: Classico = Music ::new("1795");
    
    Mu1.Gen();
    Mu1.Tempo();
  
}
