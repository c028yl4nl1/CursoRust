fn main(){
	
	let mut nome = String::from("Ola ");
	let mudar = &mut nome;

	mudar.push_str("Dev");

	println!("{}", nome);


}
