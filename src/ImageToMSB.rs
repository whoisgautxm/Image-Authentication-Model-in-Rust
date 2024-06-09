use image::{ImageBuffer, open};

fn main(){
    let img = open("../Hii.png").unwrap();
    let (x,y)= dimensions(&img);

    println!("x:{},y:{}",x,y);
}