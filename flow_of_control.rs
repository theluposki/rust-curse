use std::io;

fn convert_to_int(data_input: & String) -> i32  {
  let x = data_input.trim().parse::<i32>().unwrap();
  x
}

fn main() {
  let num1: i32 = 24;
  let num2: i32 = 24;

  // Condicionais 
  if num1 < num2 {
    println!("{} é menor que {}",num1,num2);
  }

  if num2 > num1 {
    println!("{} é maior que {}",num2,num1);
  } else {
    println!("{} é igual a {}", num2,num1);
  }

  let mut num3 = String::new();
  println!("Digite um Numero:");
  io::stdin().read_line(&mut num3).expect("Erro ao ler num3");
  
  let mut num4 = String::new();
  println!("Digite outro Numero:");
  io::stdin().read_line(&mut num4).expect("Erro ao ler num4");

  if convert_to_int(&num3) > convert_to_int(&num4) {
    print!("\nO numero {} é maior que {}", num3, num4);
  } else {
    print!("\nO numero {} é menor que {}", num3,num4);
  }
}
