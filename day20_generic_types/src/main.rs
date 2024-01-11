#[derive(Debug)]
//generic struct
struct Point<T>{
    x:T,
    y:T
}

struct NextPoint<X1, Y1>{
    x:X1,
    y:Y1
}

impl <X1, Y1> NextPoint<X1, Y1>{
    fn mix_up<X2, Y2>(self, other:NextPoint<X2, Y2>)->NextPoint<X1, Y2>{
        NextPoint{
            x:self.x,
            y:other.y
        }
    }
}

//generic methods for the struct
impl<T> Point<T>{
    fn x(&self)->&T{
        &self.x
    }

    fn y(&self)->&T{
        &self.y
    }
}

//can implement methods for specific types too
impl Point<f32>{
    fn distance_from_origin(&self)->f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//generic enum
enum Option<T>{
    Some(T),
    None
}

//generic enum
enum Result<T,E>{
    Ok(T),
    Err(E)
}

//generic function without traits. throws error on comparison of items
fn largest<T>(list:&[T])->&T{
    let mut largest = &list[0];
    for item in list{
        if item>largest{
            largest=item;
        }
    }
    largest
}
// fn largest_i32(list: &[i32])->&i32{
//     let mut largest = &list[0];
//     for item in list{
//         if item>largest{
//             largest=item;
//         }
//     }
//     largest
// }
//
// fn largest_char(list: &[char])->&char{
//     let mut largest = &list[0];
//     for item in list{
//         if item>largest{
//             largest=item;
//         }
//     }
//     largest
// }

fn main() {
    let integer = Point{x:5,y:10};
    let float = Point{x:1.0,y:4.0};
    println!("integer.x = {}", integer.x());
    println!("integer.y = {}", integer.y());
    // let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&integer);
    println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&float);
    println!("The largest char is {}", result);

    let p1 = NextPoint { x: 5, y: 10.4 };
    let p2 = NextPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
