extern crate randolib;

fn main() {
     match randolib::randomize()
     {
         Ok(()) => (),
         Err(e) => eprintln!("{:?}", e)
     }
}
