fn main(){

	let s1 = String::from("Ref");
		
	let len = len_value(&s1);

	println!("Não foi dropado s1: {} ,Tamanho len : {}", s1, len);

		
}

fn len_value(value: &String) -> usize {

	value.len()


}
