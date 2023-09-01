enum coin{
	money01, 
	money02,
	
}	
fn main(){
	

}

fn Ver_Coin(coins: coin) -> u8 {

	match coins{
	coin::money01 => 2,
	coin::money02 => 10,
	
	}
}
