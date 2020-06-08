// use codegen::gen_struct;
use codegen::gen_struct_by_conf;

mod conf;

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }
//
// impl  Person {
//     fn new(name: String, age: u32) -> Self {
//         Person { name, age }
//     }
// }


// gen_struct! {
//      Person -> {name: String, age: u32}
// }

gen_struct_by_conf!{ confs / method.toml }

const CONF_FILE_PATH : &'static str = "confs/method.toml";

fn main() {
    let person = Person::new(18,  "Alex".to_string());
    println!("{:?}", person);

    let conf = conf::Conf::read_config();
    println!("{:?}", conf);
}
