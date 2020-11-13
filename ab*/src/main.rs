/*
AFD en rust 
name and surname Alexis Antonio Balderas Aguirre
major: 160236 
speciality professor: Juan Carlos González Ibarra
institution: Universidad Politécnica de SLP
enrollment: ITI
*/

extern crate regex;
use regex::Regex;
use std::process;
use std::io;
//Definimos la funcion caracter 
fn caracter(character: char) -> i32{
    let a = Regex::new(r"a").unwrap();
    let b = Regex::new(r"b").unwrap();
    let _fin = " ";
    let _operador=["+","-","*","/"];
    
    //#comparamos si es digito o operador
    let mut s = String::from("");
	s.push(character);
    let newCharacter: &str = &s[..];
    
    //se checa si es una a o b
    if a.is_match(newCharacter)
    {
        return 0;
    }
    else 
    {
	    if b.is_match(newCharacter) 
	    {
	        return 1;
	    }
	    else 
	    {
	    	if character=='\n' || character=='\r'
		    {
		    	return 2;       //fin de la cadena
	    	}
	    }
	    //sse acaba el programa si es un caracter no valido
		println!("Error el caracter: {} no es valido",character);
		process::exit(0x0100);
    }    
    
}

fn encabezado(){
    println!("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
    body();
    }

//definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
fn contenido(q1: i32,character: char,simbolo: &str,q0: i32,stack: &Vec<char>)
{
    println! ("|      {}       |    {}    | {}|       {}       |{:?}",q1,character,simbolo,q0,stack);
    body();
}

//solo muestra la linea que se repetira cada vez que la mandemos a llamar
fn body(){
    println!("+--------------+---------+-----------+---------------+");
    }


fn main(){
    //ayuda del alumno Humberto 
    let tabla = vec![  vec! ['1','E','A'], vec! ['1','2','E'], vec! ['E','2','A'] ];

    let mut cadena= String::new(); //cadena a leer
    let mut q0: i32 = 0; //estado inicial
    let mut stack: Vec<char> = Vec::new(); //pila
    let mut pop: bool = 0; //controlador de la pila
    let mut simbolo: String = "".to_string(); //lo que lee el automata

    println!("+-------------------------------------+
    |    Ingrese una cadena a evaluar:    |
    +-------------------------------------+");
    io::stdin().read_line(&mut cadena); //se lee la cadena
    body();
    encabezado();
    contenido(0,'Ɛ',"          ",0,&stack);

    //ciclo para recorrer la cadena
    for  character in cadena.chars()
    {
        //ayuda del alumno Humberto 
        let charcaracter = caracter(character); //se comprueba si es un caracter valido
        let  q1: i32 = q0; 
        q0 = (tabla[ q0 as usize][charcaracter as usize]) as i32 - '0' as i32 ; //segun la tabla de trancision, se checa el siguiente estado
        if charcaracter == 0    //se checa si fue a
        {
            simbolo = "    a     ".to_string();
            stack.push('x');
        }
        else if charcaracter == 1 //se checa si fue a
        {
            simbolo = "    b     ".to_string();
            if stack.is_empty() 
            {  
                pop = 1; //antes de hacer pop, se indica que la pila ya esta vacia
            }
            stack.pop();
        }
        else if charcaracter == 2 //fin de la cadena
        {
            simbolo = "Fin Cadena".to_string();
        }

        if q0==21 //estado de error de la cadena
        {
            let mut character_imp = character;
            if character_imp == '\r'
            {
                character_imp = ' ';
            }

            println! ("|      {}       |    {}    |{} |     Error     |",q1,character_imp,simbolo);
            body();
            println! ("|              Cadena No Valida                   |");
            println! ("+----------------------------------------------------+");
            process::exit(0x0100);
        }

        if q0 == 17  //estado final de la cadena
        {
            println! ("|      {}       |         |Fin Cadena |               |",q1);
            body();
            if stack.is_empty() && pop != 1
            {
                println! ("|                Cadena Valida                   |");
                println! ("+----------------------------------------------------+");
            }
            else
            {
                println! ("|              Cadena No Valida                   |");
                println! ("+----------------------------------------------------+");
            }
            process::exit(0x0100);
        }
        contenido(q1,character,&simbolo,q0,&stack); 
    }  
}