#[derive(Debug)]
pub trait Sub{
	fn Su(&self) -> bool{
	true
}
	}
#[derive(Debug)]
pub struct Payload<T,Y>{
	
	pub ip: T,
	pub host: T,
	pub buffer:Y ,
	
}


impl<T:std::fmt::Debug,Y: std::fmt::Debug> Sub for Payload<T,Y>{
	fn Su(&self) -> bool {
	println!("{:?}", self);
	true
	}

}
fn main(){

	println!("Traits");

}
