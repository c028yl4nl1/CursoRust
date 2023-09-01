#[derive(Debug)]
struct Args2<T>{
	Arg1: T,
	Arg2: T,
}

impl<T: std::fmt::Debug > Args2<T>{
	fn Somar(self) -> T {
	println!("Ola {:?}", self.Arg1);
	self.Arg2
	}
}

fn main(){

	let Valor = Args2{Arg1: 2, Arg2: 3};

	println!("Pronto {:?}",Valor.Somar() );

}


