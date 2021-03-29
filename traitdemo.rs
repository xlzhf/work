fn main() {

    let square = Square {                 //创建一个正方形结构体
        side : 5.0,
    };

    notify(&square);                      //调用接口的方法

    let triangle = Triangle {
        bottom : 5.0,
        high : 3.5,
    };
    
    notify(&triangle);

    let circular = Circular {
        radius : 7.1,
    };
    
    notify(&circular);
    
}

pub trait Shape{                         //定义一个图形接口
    fn area(&self) -> f32;               //定义一个未具体实现的方法
}

struct Square {                         //创建一个正方形结构体
    side : f32,                         //声明一个成员变量
}

impl Shape for Square{                  //定义结构体对结构的实现
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

struct Triangle {
    bottom : f32,
    high : f32,
}

impl Shape for Triangle{
    fn area(&self) -> f32 {
        self.bottom * self.high / 2.0
    }
}

struct Circular {
    radius : f32,
}

impl Shape for Circular{
    
    fn area(&self) -> f32 {
        const PI: f32 = 3.14159265;
        self.radius * PI
    }
}

pub fn notify<T:Shape>(item: &T){
    println!("area is {}", item.area());
}