fn main() {
    let a = [10, 20, 30, 40, 50]; 
    let sum = plus_all(&a).map(|u| u);       //调用方法求出a数组的总和

    println!("{:?}", sum);
}

fn plus_all(x:&[u32]) -> Option<u32>{            //创建一个数组求和方法

    let mut sum = 0;                             //声明一个sum的初始值
    for i in x.iter(){                           //遍历数组
        if sum < 2147483647 {                    //判断sum是否小于u32的最大值
            sum = sum + i;
        }
        else{                                    //若大于返回None
            return None;
        }    
    }
    
    match Some(sum) {
        Some(i) => Some(i),
        None => None,
    }
}