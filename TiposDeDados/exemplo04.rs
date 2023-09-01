fn main() {


	println!("___");

	let tupla: (i32, i32, i32, i32) = (10,11,21,12); // 4 elementos de i32

	let tupla_sem_tipagem = (1,23,4,3,32,3232);
	
	// --- Array --- //

	let array_0 = ["carro", "moto", "bike"]; // No array não posso add outros tipos de dados juntos.
	let array_2: [i32; 4] = [1121,11,121,12];	

	let tupla_ = (1,2,3,4);

	let (p,x,y,z) = tupla_;
	
	println!("Valor de um Objeto da tupla : {}",x);

	println!("{}", tupla_.0);
}
