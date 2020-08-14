use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let mut sn = String::from(s);
  let short_s = sn.split_off(1);
  println!("The Rust function say() received {}", s);
  println!("{}!", s);
  println!("{}, {}, bo-b{}", s, s, short_s);
  println!("Bonana-fanna fo-f{}", short_s);
  println!("Fee fi mo-m{}", short_s);
  println!("{}!", s);
  let r = String::from("hello ");
  return r + s;
}
