#[derive(Debug)]
enum TypesIp{
        V6,
        V4, 
}
fn main(){

        println!("Enum exemplo 2 ");

        let ipv6 = TypesIp::V6;
        let ipv4 = TypesIp::V4;
	
	Values(&ipv4);
}


fn Values(instacia: &TypesIp){
	
	match instacia {
	TypesIp::V4 => println!("Valor v4"),
	TypesIp::V6 => println!("Valor v6"),
	}
}
