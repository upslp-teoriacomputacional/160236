NFA en rust 
Se utilizo VS code para realizarlo pero se compilo con cargo

extern crate regex; <--- esto lo que hará sera crear las expresiones regulares

Las librerias a utilizar son:

use std::io;        Las entradas por teclado
use regex::Regex;   Las expresiones regulares
use std::process;   Los procesos

let a = Regex::new(r"a").unwrap();  crea la expresion regular para a
let b = Regex::new(r"b").unwrap();  crea la expresion regular para b

unwrap hara que devuelva la cadena o un error en caso de no tenerla, en este caso siempre se tendra una cadena

if a.is_match(new_character) comparara si es una a o una b (b.is_match) y regresa un entero que representa el estado segun corresponda.
    {..}
    
process::exit(0x0100); terminara el programa si no es valido el caracter.

let tabla: Vec<Vec<char>>;
    tabla = vec![...]
  
  Se crea la tabla de estados, representado en rust como un vector de vectores [[], [], [], [], []]
  
  io::stdin().read_line(&mut cadena); va a leer la entrada por teclado del usuario.
  
  
