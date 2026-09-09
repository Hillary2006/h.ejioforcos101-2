fn main() {
   let zikora_total:f64 = 2.0 * 450000.0;
   let zara_total:f64 = 1.0 * 1500000.0;
   let zitel_total:f64 = 3.0 * 750000.0;
   let ojinny_total:f64 = 3.0 * 2850000.0;
   let janet_total:f64 = 1.0 * 250000.0;

   //sum
   let sum_total:f64 = zikora_total + zara_total + zitel_total + ojinny_total + janet_total;
   println!("{}", sum_total);
   let qty_total:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;

   //average
   let average:f64 = sum_total / qty_total;
   println!("{}", average);  
}
