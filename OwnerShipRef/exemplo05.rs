fn main(){

	let s1 = String::from("S1");

	let s2 = &s1;
	
	let s3 = &s1;
	
	// Erro //let s4 = &mut s1;
	println!("S2: {}, S3: {}", s2,s3);
	

	
}
