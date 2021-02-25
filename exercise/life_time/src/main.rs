struct Person{
    name:String
}

struct Company<'z>{
    name: String,
    ceo: &'z Person
}

impl <'a> Company<'a>{
    fn who_ceo(&self){
        println!("ceo is {}.",self.ceo.name);
    }
}

fn main() {
    let mut p = Person{name:"john".to_string()};
    let mut c = Company{name:"Vodafone".to_string(), ceo:&p};
    c.who_ceo();
}
