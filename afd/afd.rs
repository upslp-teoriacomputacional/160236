/*
AFD en rust 
name and surname Alexis Antonio Balderas Aguirre
major: 160236 
speciality professor: Juan Carlos González Ibarra
institution: Universidad Politécnica de SLP
enrollment: ITI
*/


use std::process;
use std::io;
use std::str;

//Definimos la funcion caracter 
fn caracter(character: char) -> i32{
    let mut Fin = " ";
    let operador=["+","-","*","/"];
    
    //#comparamos si es digito o operador
    let mut s = String::from("");
	s.push(character);
    let newCharacter: &str = &s[..];
    
    if character.is_digit(10){
			return 0;                   //  Si es digito devuelve un cero          
    }
    else{
        if character == '+' || character == '-' || character == '*' || character == '/' {
            return 1;
            }
        else{
            if newCharacter==Fin {
                return 2;
                }
        }
        //#si no es ni un digito ni un operador entonces es un caracter no validp
        println!("Error el caracter: {:?}",character); print!("no es valido");
        process::exit(0x0100);
        }
}

fn encabezado(){
    println!("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
    body();
    }

//definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
 fn contenido(estadosig: i32, character: char, simbolo: &str, estado: i32){
     println! ("|      {}       |    {}    |  {} |      {}        |",estadosig,character,simbolo,estado);
    body();
}

//solo muestra la linea que se repetira cada vez que la mandemos a llamar
fn body(){
    println!("+--------------+---------+-----------+---------------+");
    }


fn main(){
    //let tabla=[["1","E","E"],["E","2","E"],["3","E","E"],["E","E","A"]]; tabla original
    let tabla = [[1,5,5], [5,2,5], [3,5,5], [5,5,0]];  //  se cambian los valores de A y E por 0 y 5
    let mut  tipo: String = "".to_string(); 
    let mut  Fin: String = "".to_string();  
    let mut estado = 0;      
            

    let mut estado = 0;
    let mut cadena= String::new();	
    let mut estadosig;
    println!("+-------------------------------------+
    |    Ingrese una cadena a evaluar:    |
    +-------------------------------------+");
    io::stdin().read_line(&mut cadena);
    body();
    encabezado();

    //#ciclo para recorrer la cadena
    for  character in cadena.chars() {
        estadosig = estado;
        
        //  Llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut charcaracter: i32 = caracter(character);
        //#llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let charcaracter= caracter(character);
        if charcaracter == 0{                   //  Si es digito la variable tipo guarda "Digito"
      	    tipo = "Digito".to_string();
      	}
      	else if charcaracter == 1{              //  Si es operador la variable tipo guarda "Operador"
      	    tipo = "Operador".to_string();
      	}
      	else if charcaracter == 2{              //  Si es vacio la variable tipo guarda "Vacio"
      	    tipo = "Error".to_string();
      	}
        
        //#guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
        estado = tabla[estado as usize][charcaracter as usize];
    
      
        //#Si el valor obtenido es una E imprimimos cadena no valida
         if estado == 69{                        //  Si el estado en el que estamos es el E significa que la cadena no es valida
            println!("|    {}    |  {}    |     {}     |     {}      |", estadosig, character, tipo, estado);
            body();
            println!("|              Cadena No Valida                       |
    +----------------------------------------------------+");
            process::exit(0x0100);
        }
    }

    //#al concluir si el estado no es 3 que es el de aceptacion imprimimos cadena no valida    
    if estado!=3 {
            println!("|              Cadena No Valida :(                   |
    +----------------------------------------------------+");
    }
    //si el estado es 3 es una cadena de aceptacion
    if estado==3 {
        println! ("|      {}       |         |Fin Cadena |               |",estado);
        body();
        println!("|                Cadena Valida                       |
    +----------------------------------------------------+");
    }
}
