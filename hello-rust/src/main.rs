// //https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
//
//
// struct node{ //these are the independent nodes that store the value and whatever else you would like specifically
//     x: u8
// }
//
// impl node{
//     fn new(x: u8) -> Self{ // creates "new" functionality
//         node{
//             x
//         }
//     }
//     fn print_number(&self) -> u8{
//         self.x;
//     }
// }
//
// fn main() {
//     println!("Alright, time to get down");
//         let mut list = vec![
//         node::new(11),
//         node::new(15),
//         node::new(31),
//         node::new(52),
//         node::new(40),
//         ]; //creation of nodes within list (vector)
//
//
//         node::print_number
//         //node.sort();
// }






#![allow(dead_code)]

#[derive(Debug)]
struct car
{
    color: String,
    year: u32,
}

fn main()
{
    let color = String::from("nice");
    let year = 1925;
    let Honda = car{color, year};
    println!("{:?}", Honda);
}
