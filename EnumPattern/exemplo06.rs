fn main(){
	
	let a: Option<i32> = Some(5);

	let b = 5;


	match a {
	None => println!("Sem valor"),
	Some(valor) => println!("O valor de B + A = {} ", valor + b),


	}

}
