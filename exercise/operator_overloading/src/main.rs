#[allow(unused_variables)]
#[allow(dead_code)]
use core::ops::{Neg, AddAssign, Add};
use std::cmp::PartialEq;

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl <T> Complex<T>{
    fn new(re:T, im:T)->Complex<T>{
        Complex::<T>{ re, im}
    }
}

impl <T> Add for Complex<T>
    where T:Add<Output=T>{
    type Output = Complex<T>;
    fn add(self, rhs:Self)->Self::Output{
        Complex::<T>{
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl <T> AddAssign for Complex<T> where T:AddAssign<T>{
    fn add_assign(&mut self, rhs: Complex<T>){
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl <T> Neg for Complex<T> where T:Neg<Output = T>{    
    type Output = Complex<T>;
    fn neg(self) -> Self::Output{
        Complex::<T>{
            re: -self.re,
            im: -self.im
        }
    }
}

impl <T> PartialEq for Complex<T> where T:PartialEq{
    
    fn eq(&self, rhs: &Self) -> bool{
        (self.re == rhs.re)&&(self.im == rhs.im)
    }
}

fn main() {    
    let mut x = Complex::new(3.5,4.0);
    let mut y = Complex::new(3.2,4.1);
    // x+=y;
    // println!("{:?}",x);
    // println!("{:?}",x+y);
    // println!("{:?}",-y);
    println!("{}",x==y);
    println!("{}", x==x);
    
}
