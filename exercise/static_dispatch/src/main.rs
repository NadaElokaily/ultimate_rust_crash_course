trait Printable{
    fn format(&self)->String;
}

impl Printable for i32{
    fn format(&self)->String{
        format!("i32: {}", *self)
    }
}

impl Printable for String{
    fn format(&self)->String{
        format!("String: {}",*self)
    }
}

fn print_anything<T:Printable>(x :T){
    println!("{}", x.format());
}


fn main() {
    let i = 5;
    let s = "hello".to_string();
    print_anything(i);
    print_anything(s);

}
