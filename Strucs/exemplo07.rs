fn main(){
	let rect = (10, 30);
	println!("{}",wight(rect));



}

fn wight(valor: (usize , usize)) -> usize {
	valor.0  * valor.1
	
}
