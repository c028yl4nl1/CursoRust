#[derive(Debug)]
struct User{
	email: String,
	nome: String,
	ativado: bool,
}

fn main()	{
	
	
	let info = build_info(String::from("Lanby@gmail.com"),String::from("lanby"), false );
	
	println!("{:#?}", info);
	

}

fn build_info(email: String, nome: String , ativado: bool) -> User{
	
	User{

	email: email,
	nome: nome,
	ativado: ativado,

	}
}
