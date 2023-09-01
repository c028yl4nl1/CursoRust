use std::collections::HashMap;

fn main(){
	let mut add = HashMap::new();
	let mut cont = 1000;
	add.insert("caios", 100);
	while cont > 0 {  // Exemplo abrindo um arquivo 
	cont -= 1;
	if let Some(valor) = add.get("caios"){
	println!("Valor: {}", valor);
	}

	else {
	println!("Não Tem valor");
	}
	}



}
