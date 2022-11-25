use std::io;

fn main() {
    println!("\t<calc rust>");
	println!("pressione 'h' para receber ajuda e 'q' para sair\n");
	getting_arg();
}

fn getting_arg() {
	loop {
		println!("<insira argumento>");
		let mut arg = String::new();
		
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
		
		if arg.trim() == "h" {
			get_help();
			break;
		} else if arg.trim() == "q" {
			println!("\naté mais...");
			break;
		} else if arg.trim() == "c +" {
			get_calc_sum();
			break
		} else if arg.trim() == "c -" {
			get_calc_sub();
			break
		} else if arg.trim() == "c /" {
			get_calc_div();
			break
		} else if arg.trim() == "c *" {
			get_calc_mul();
			break
		}
	}
}

fn get_help() {
	println!("\n*   h   : mostra ajuda
*   q   : sair
*   c + : calcula soma
*   c - : calcula subtração
*   c / : calcula divisão
*   c * : calcula multiplicação");
	getting_arg();
}
fn get_calc_sum() {
	let mut arg	 = String::new();
	let mut num1: f64;
	let num2: f64;
	loop {
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
			
		let arg: f64 = match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
		};
		
		num1 = arg;
		
		let mut arg = String::new();
		
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
			
		let arg: f64 = match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
		};
		
		num2 = arg;
		
		let res = num1 + num2;
		
		println!("{num1} + {num2} = {res}\n");
		getting_arg();
		break;
	}
}
fn get_calc_sub() {
	let mut arg	 = String::new();
	let mut num1: f64;
	let num2: f64;
	loop {
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
			
		let arg: f64 = match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
		};
		
		num1 = arg;
		
		let mut arg = String::new();
		
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
			
		let arg: f64 = match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
		};
		
		num2 = arg;
		
		let res = num1 - num2;
		
		println!("{num1} - {num2} = {res}\n");
		getting_arg();
		break;
	}

}
fn get_calc_div() {
	let mut arg	 = String::new();
	let mut num1: f64;
	let num2: f64;
	loop {
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
			
		let arg: f64 = match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
		};
		
		num1 = arg;
		
		let mut arg = String::new();
		
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
			
		let arg: f64 = match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
		};
		
		num2 = arg;
		
		let res = num1 / num2;
		
		println!("{num1} / {num2} = {res}\n");
		getting_arg();
		break;
	}
		
}
fn get_calc_mul() {
	let mut arg	 = String::new();
	let mut num1: f64;
	let num2: f64;
	loop {
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
			
		let arg: f64 = match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
		};
		
		num1 = arg;
		
		let mut arg = String::new();
		
		io::stdin()
			.read_line(&mut arg)
			.expect("falha ao ler a linha");
			
		let arg: f64 = match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
		};
		
		num2 = arg;
		
		let res = num1 * num2;
		
		println!("{num1} * {num2} = {res}\n");
		getting_arg();
		break;
	}
}
