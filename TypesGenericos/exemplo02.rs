struct chamar<T>{
 a: T,
}
impl<T> chamar<T>{
	fn a(self) -> T {
	self.a 
	}
	}
fn main(){

	let a  =  chamar{a: 3};
	let y  = chamar{a: 3}.a;
	println!("{}", a.a());
	//println!("{}", a.a());

	// Move
}
