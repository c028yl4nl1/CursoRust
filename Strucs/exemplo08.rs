
#[derive(Debug)]
struct Valores{
	valor01: u32,
	valor02: u32,
}

fn main(){

	let add_valor = Valores{valor01: 5 ,valor02: 5 };

	let recv = somar(&add_valor);

	println!("Valor01: {} *  Valor02: {} = {} ",add_valor.valor01 , add_valor.valor02 , recv );

	dbg!(add_valor);
}


fn somar(valores: &Valores) -> u32 {
	
	valores.valor01 * valores.valor02

}
