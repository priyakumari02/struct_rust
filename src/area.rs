#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
}

fn main(){
    let rect=Rectangle{
        width:30,
        height:50
    };
    let rect2=Rectangle{
        width:20,
        height:40
    };
    let rect3=Rectangle{
        width:50,
        height:60
    };
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    println!("rect1 is {:#?}", rect);
    // println!("Area of rectangle : {}",area(&rect));
    println!("Area of rectangle : {}",rect.area());
}
/*
fn area(rect:&Rectangle)->u32{
    rect.width*rect.height
}
*/