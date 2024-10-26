fn main() {
	let principal:f64 = 520_000_000.0;// loan amount in naira
	let rate:f64 = 10.0; // Intrest rate per annum
	let time:i32 = 5; // Number of years
	// Formula: A = P(1 + R/100)^n
	let amount = principal * (1.0 + rate / 100.0).powi(time);
	//Compound Intrest: CI = A-P
	let compound_intrest = amount - principal;
	println!("Total amount after {} years: \u{20A6}{:.2}", time, amount);
	println!("compound intrest: \u{20A6}{:.2}", compound_intrest);
}