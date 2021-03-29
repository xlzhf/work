fn main() {
    let redLight = TrafficLight::Red;        
    let greenLight = TrafficLight::Green;
    let yellowLight = TrafficLight::Yellow;
    println!("redLight is {}", redLight.time());
    println!("greenLight is {}", greenLight.time());
    println!("yellowLight is {}", yellowLight.time());
}

enum TrafficLight {                                        //创建一个交通灯枚举型
    Red,
    Green,
    Yellow
}

impl TrafficLight {                                    //对交通灯的变体做模式匹配返回不同的结果
    fn time(&self) -> u8 {
      match &self {
        TrafficLight::Red => {
            60
        }
        TrafficLight::Green => {
            30
        }
        TrafficLight::Yellow => {
            5
        }
      }
    } 
}
