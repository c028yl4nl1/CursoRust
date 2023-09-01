#[derive(Debug)]
enum Help{
	Ajuda(bool), 
	Ativo(bool),
	Text(String),
		
}

fn main(){
	let _lan: Vec<Help>  = vec![Help::Ajuda(true)];
	println!("{:?}", _lan); 
	
	}
