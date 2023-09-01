use std::collections::HashMap;

fn main(){

	{
	println!("HashMap");
	}
	let mut a = HashMap::new();
	let u: Vec<i32> = vec![1212,12,12,12,];
	a.insert(String::from("a"),1);
	a.insert(String::from("a"),32,32,23,22,2);
	
	println!("{:?}", a);
	//let a: Vec<_> = vec![1,2,3,3,4];

	let buscar = a.get(&"a".to_string());
	println!("{:?}",buscar);
}
