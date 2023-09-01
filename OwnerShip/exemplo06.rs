fn main() {
	let nome = String::from("Maicon");
	

	let (nome_, tamanho) = Len_var(nome);
	println!("Nome: {} , Tamanho: {}", nome_ , tamanho);

	//
}


fn Len_var(valor: String) -> (String , usize){

	let len = valor.len();

	(valor, len)

}
