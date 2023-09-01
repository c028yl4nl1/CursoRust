#[derive(Debug)]
enum Types{
	V6(String),
	V4(String),
}

fn main(){
	let valuestring = String::from("Ipv6");
	let valor = Types::V6(valuestring);
	
	println!("Values: {:?}", &valor);


	
			
}



