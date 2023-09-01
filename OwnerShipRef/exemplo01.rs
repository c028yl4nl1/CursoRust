fn main() {


	println!("Referecias");
	
	let s1 = String::from("Ref");
	let (s2 , len) = calculet_len(s1);
	
	println!("Valor s2: {}", s2);

		

	// Aqui não tem Ref
}


fn calculet_len(value: String) -> (String , usize){

	let len = value.len();
	(value, len)
}
