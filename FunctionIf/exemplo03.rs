fn main() {


	let nome: String = String::from("Vitor");
	
	let nome_objeto: &str =	"Caneta";

	let chave: char = '5';

	let verdadeiro = false;

	Function_exemplo(nome,nome_objeto, chave, verdadeiro);

}

fn Function_exemplo(nome: String , objeto: &str , chave: char, verificar: bool){

	println!("Nome: {}, Objeto: {}, Chave: {}, Verificar: {}", nome, objeto, chave, verificar);	


}
