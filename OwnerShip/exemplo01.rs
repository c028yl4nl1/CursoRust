fn main(){

	println!("OwnerShip");
	//
	// Cada vez que um bloco de dados for saido do escolpo ele sera limpo da memoria
	//
	
	{
	let a = "Stack";	
	let b = String::from("OwnerShip");	
	}

	println!("Memoria: <Drop>");
	
}
