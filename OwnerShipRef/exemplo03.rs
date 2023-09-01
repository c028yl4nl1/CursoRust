fn main(){

	println!("Ref");
	let s1 = String::from("Nome");
	Ver_nome(&s1);

	
}

fn Ver_nome(nome: &String){
	println!("Boa noite: {}", nome);



}
