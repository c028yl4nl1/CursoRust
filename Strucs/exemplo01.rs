#[derive(Debug)]
struct Users{
	ativado: bool,
	email: String,
	saldo: i32 ,

}

fn main(){
	println!("Tipos de Structs");
	
	let info = Users{ativado: false , email: String::from("teste@gmail.com"), saldo: 102};
	
	println!("Struct: {:#?}",info);
	

}
