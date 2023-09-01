#[derive(Debug)]
struct Users{
        ativado: bool,
        email: String,
        saldo: i32 ,

}

fn main(){
        println!("Tipos de Structs");

        let mut info = Users{ativado: false , email: String::from("teste@gmail.com"), saldo: 120};

        println!("Ativado: {}, email: {}, saldo: {}", info.ativado,info.email,info.saldo);
	
	info.email = String::from("caio@gmail.com");

	println!("Ativado: {}, email: {}, saldo: {}", info.ativado,info.email,info.saldo);	

}









