
//Box Smart Pointer

//#[derive(Debug)]
//enum List{
//    Cons(i32, Option<Box<List>>),
//}

//fn main() {
    //let x = 0.625;
    //let y = Box::new(x);
    //let z = &x;

//    let list= List::Cons(1, Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))));
//    println!("{:?}", list);
//}

struct Huge_Data;
struct Small_Data;
trait Storage {}

impl Storage for Huge_Data {}
impl Storage for Small_Data{}

fn main() {
    let data_1 = Huge_Data;
    let data_2 = Box::new(Huge_Data);

    let data_3 = data_1;
    let data_4 = data_2;

    let data_5 = Box::new(Small_Data);

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];
}