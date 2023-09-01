#[derive(Debug)]
enum Option<T>{

	None,
	Some(T),	

}
fn main(){

	let valor = Some(String::from("Curso"));

	if let Some(valor) = valor{


	println!("Valor: {}", valor);
}


	}
