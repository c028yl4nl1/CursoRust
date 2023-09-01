#[derive(Debug)]
struct Rectagle {
	y: u32 ,
	x: u32,

}

impl Rectagle{

	fn area(&self) -> u32  {
	self.x * self.y
	}

}


fn main(){

	let ver_tamanho_da_area = Rectagle{y: 10 , x: 30};
	println!("Tamanho: {}", ver_tamanho_da_area.area());
}
