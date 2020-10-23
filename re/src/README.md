RE en rust Se utilizo VS code para realizarlo pero se compilo con cargo

extern crate regex; <--- esto lo que hará sera crear las expresiones regulares

let mut tokens: Vec<String> = Vec::new(); <--- se define un vector de strings

let  source_code = "int result = 1;".split(" "); <--- transforma de una cadena a una lista de palabras

Se definen los tipos de valores que pueden ingresar
let types: [&str; 3] = ["str", "int", "bool"];

En este punto se crean las expresiones regulares y el unwra
let digito = Regex::new(r"[0-9]").unwrap();
let letras1 = Regex::new(r"[a-z]").unwrap();
let letras2 = Regex::new(r"[A-Z]").unwrap();
let operador = Regex::new(r"(\+|\-|\*/)").unwrap();

.unwrap(); hara que devuelva la cadena o un error en caso de no tenerla, en este caso siempre se tendra una cadena

types.contains(&word) <-- checa si contiene lo que tenga la variable word

word.as_bytes() <-- los bytes que tiene

for word in source_code <-- va a recorrer todas las palabras que hay en la cadena de inicio

.remove(word.len() - 1) <-- remueve el ultimo en la cadena

w.push(w_chars); <-- lo agrega al final del vector

w.reverse(); <-- los voltea de posicion
w.pop(); <-- remueve el ultimo elemento
w.reverse(); <-- los vuelve a voltear

