fn main(){

	let mut count = 0;
	'counting_loop: loop {
	
	   println!("Estou dentro de um Loop main");
		
	   let mut roming = 10;

	
	loop {
	println!("roming: {}", roming);
	if roming == 9 {
		break;
	}
	if count == 2 {
		break 'counting_loop;
	
	}
	roming -= 1;
	}

	count +=1;
}	
	println!("Count: {}", count);		
}
