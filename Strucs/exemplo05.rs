#[derive(Debug)]
struct User{
	email:String, 
	pass: String,

}

fn main(){
	let info01 = build_info(String::from("teste@gmail.com"), String::from("teste123"));
	
	let info02 = User{email: String::from("Kaue@gmail.com"), pass: info01.pass };


	println!("{:?}", info02);
}

fn build_info(email: String, pass: String) -> User {

	User{
	email:email, 
	pass: pass,
}

}
