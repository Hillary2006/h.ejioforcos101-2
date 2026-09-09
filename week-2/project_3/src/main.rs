fn main() {
  let t:f64 = 3.0;
  let r:f64 = 5.0;
  let p:f64 =210000.0;

  //formula
  let a:f64 = p * (1.0 - (r / 100.0)).powf(t);
  println!("value of the tv after {}years:{:.2}", t, a);  
}
