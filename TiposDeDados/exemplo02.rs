fn main(){

	println!("Conta");

	let valor_i32: i32 = 10; // i32
	let valor_u8: u8 = 10; // u8
	
	
	// i32 + u8 = Panic , Pois não posso somar os tipos diferente.

	// "as" Estou forçando entre Aspas
	println!("A Soma entre {} + {} = {} ", valor_i32 , valor_u8 , (valor_i32 + valor_u8 as i32));
	
	let valor_f32: f32 = 0.21 + 1.0;
	let valor_f64: f64 = 0.21 + 1.0;

	println!("A soma de {} entre {} = {}", valor_f32, valor_f32 , (valor_f32 + valor_f32)); 

	// f64 tem mais precisão



	println!("A soma de {} entre {} = {}", valor_f64,valor_f64,(valor_f64 + valor_f64));
	 
	//  f32 

	// --------- //
	   // Se eu não passa o tipo , O padrão vai ser f64.   //


}
