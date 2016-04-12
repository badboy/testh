#![feature(non_ascii_idents)]
#![feature(plugin, custom_attribute)]
#![plugin(testh)]

#[tesŧ]
fn it_really_works() {
    assert!(true);
}

#[cfg(test)]
mod tesŧ {
    #[tesŧ]
    fn it_works() {
    }
}

fn main() {
    println!("hello world");
}
