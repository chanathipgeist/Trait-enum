trait Instrumental {
    fn new() -> Self;
    fn play(&self) -> String;
    fn maintenance(&self) -> String;
}

struct Guitar { 
   pub name:String,
   pub sound:String,
}

struct Piano { 
    pub name:String,
    pub sound:String,
    pub price:u32
 }

impl Instrumental for Guitar  {
    fn new() -> Self{
        Guitar{ 
            name:String::from("Guitar"),
            sound:String::from("Nice")
    }
    }
    fn play(&self) -> String{
        let gui_p = String::from("deed");
    return gui_p
}
fn maintenance(&self) -> String{
    let gui_m = String::from("KwanTeePanunk");
    return gui_m
        
}
}
impl Instrumental for Piano{
    fn new() -> Self{
        Piano{ 
            name:String::from("Piano"),
            sound:String::from("Too much sound"),
            price:32000
    }
    }
    fn play(&self) -> String{
        let gui_p = String::from("KodLim");
        return gui_p
}
fn maintenance(&self) -> String{
    let gui_m = String::from("Paaklum");
    return gui_m
}
}
fn main() {
    let g:Guitar = Instrumental::new();
    let p:Piano = Instrumental::new();
    println!("---------------------------------------------------------------------");
    println!("This is {} Sound is {} If you want to play you can {} and you must keep by {}",g.name,g.sound,g.play(),g.maintenance());
    println!("---------------------------------------------------------------------");
    println!("This is {} Sound is {} If you want to play you can {} and you must keep by {}",p.name,p.sound,p.play(),p.maintenance());

}
