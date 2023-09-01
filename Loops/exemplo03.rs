fn main(){
		
	let mut dinheiro = 100;
	let Pagar_dinheiro = loop{
	if dinheiro > 50 {

		// pagar ernegia.
		dinheiro -= 50;
	}
	
	if dinheiro > 30 {

		// pagar agua 
		dinheiro -= 30;
}

	if dinheiro <= 20 {
		// Gasta com besteira
		break 20;
	}

};

	println!("Sobrou: {}", dinheiro);


}
