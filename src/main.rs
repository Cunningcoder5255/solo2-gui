extern crate solo2;
use solo2::{apps::Oath, Select, Solo2, UuidSelectable};

fn main() {
    let device: solo2::Device = solo2::Device::list().swap_remove(0);
    let mut solo2: Solo2 = device.into_solo2().unwrap();
    let mut app = Oath::select(&mut solo2).unwrap();
    let list = app.list().unwrap();
    for string in list.iter() {
        println!("{string}");
    }
}
