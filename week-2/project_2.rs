fn main() {
 //sales data:(quantity,amount)
 let t:f64 = 450_000.00;
 let tq:f64 =2.0;
 let m:f64 = 1_500_000.00;
 let mq:f64 = 1.0;
 let h:f64 = 750_000.00;
 let hq:f64 = 3.0;
 let d:f64 = 2_850_000.00;
 let dq:f64 =3.0;
 let a:f64 = 250_000.00;
 let aq:f64 = 1.0;

 //calculate total sales
 let total_sum = t*tq + m*mq +h*hq + d*dq + a*aq;
 //calculate average sales
 let average = total_sum / (tq + mq + hq + dq + aq);
 //print result
 println!("Total Sale Amount: {:.2}", total_sum);
 println!("Average Sales Amount:{:.2}", average);
}