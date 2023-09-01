fn main(){

	println!("OwnerShip em Função");

	let var: String = String::from("Helo Devs");

	Function_String(var); // Movi o Valre
	
	//  Erro  //println!("Value Var: {}", var); // ownership Moved valor
	//////////

	let var: String  =  String::from("Helo Devs");

	Function_String(var.clone());

	println!("Value Var: {}", var);

}

fn Function_String(value: String) {


	println!("Value: {}", value);
	
}
