fn main() {
	let p:f32  = 520000000.0;
	let r:f32 = 10.0;
	let n:f32 = 5.0;

	let a:f32 = p * (1.0 + (r / 100.0)).powf(n);
	let ci: f32 = a - p;

	println!("The compund interest is N{}", ci );
}