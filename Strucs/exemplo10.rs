#[derive(Debug)]
struct Object{
	nome: String ,
	sobrenome: String,

}

impl Object{

   fn nome(&self) -> bool {
	self.nome.len() < 0 // Sempre vai dar false
}
}


fn main(){

	let nome_bool = Object{nome: String::from("Teste") , sobrenome: String::from("Lang")};

	println!("Nome: {}", nome_bool.nome);
}
