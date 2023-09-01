fn main(){
	{
	let s1 = "Stack";

	let s2 = s1;
	}
	let mut s1 = "Stack";
	let s2 = s1;
	// Meu teste logico
	println!("Valor S1: {}", s1); // Result > Stack
	println!("Valor S2: {}", s2); // Result > Stack
	s1 = "stack2";
	println!("Valor s1: {}",s1);
	println!("Valor s2: {}",s2);	
	
	// Muito Estranho 

	// https://rust-br.github.io/rust-book-pt-br/ch04-01-what-is-ownership.html
}
