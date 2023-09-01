use std::fs::File;
use std::process::exit;
use std::io::Read;
fn main(){

    let  arq = File::open("lanby.txt");

    let mut arq = match arq{
        Ok(conteudo) => conteudo,
        Err(merda) => {
        println!("Erro");
        exit(1);

        }
    };
    let mut buffer_String: String = String::new();
    let buffer = arq.read_to_string(&mut buffer_String);
	println!("{}", buffer_String.trim());

}