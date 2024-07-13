/*
https://contest.yandex.ru/contest/19811/problems/B/
*/

fn main() {

	let mut  investigated_string	=	std::string::String::new();

	std::io::stdin().read_line( &mut investigated_string ).expect("ERR : 1");


	match  investigated_string.as_bytes().windows(2).filter(|elem| elem[0] == elem[1]).min() {
	
		None =>	{
		
			match  investigated_string.as_bytes().windows(3).filter(|elem| elem[0] == elem[2]).min() {

				None =>	println!("-1"),
				
				Some(element_3) => 
					println!(
						"{}{}{}", 
						element_3[0] as char, 
						element_3[1] as char,
						element_3[2] as char
					),
			
			}
			
		},
	
		Some(elements) => 
			println!(
				"{}{}", 
				elements[0] as char, 
				elements[1] as char,
				)
		,
	}
	
	return;


}
