/*
Conjuntos en rust 
name and surname Alexis Antonio Balderas Aguirre
major: 160236 
speciality professor: Juan Carlos González Ibarra
institution: Universidad Politécnica de SLP
enrollment: ITI
*/

//Definimos la funcion caracter 
fn caracter(character: char) -> char{
    let simbolo="";
    let Fin=' ';
    let digito=["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let operador=["+","-","*","/"];
    
    //#comparamos si es digito o operador
    if character == '0' || character == '1' || character == '2' || character == '3' || character == '4' || character == '5' || character == '6' || character == '7' || character == '8' || character == '9' {
        simbolo=" Digito ";
        return '0';
        }
    else{
        if character == '+' || character == '-' || character == '*' || character == '/' {
            simbolo="Operador";
            return '1';
            }
        else{
            if character==Fin {
                return '2';
                }
        }
        //#si no es ni un digito ni un operador entonces es un caracter no validp
        println!("Error el caracter: {:?}",character); print!("no es valido");
        }
}

fn encabezado(){
    println!("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
    body();
    }

//definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
fn contenido(estadosig: char,character: char,simbolo :char,estado: char){
    //println!("|     ",estadosig,"      |  ",character,"    |",simbolo," |     ",estado,"       |")
    body();
}

//solo muestra la linea que se repetira cada vez que la mandemos a llamar
fn body(){
    println!("+--------------+---------+-----------+---------------+");
    }


fn main(){
//Este es la tabla de transiciones del automata AFD creado
    let tabla=[["1","E","E"],["E","2","E"],["3","E","E"],["E","E","A"]];
    let estado = "0";
    let mut cadena;
    let mut estadosig;
    println!("+-------------------------------------+
    |    Ingrese una cadena a evaluar:    |
    +-------------------------------------+");
    //cadena = input();
    body();
    encabezado();

    //#ciclo para recorrer la cadena
    for  character in cadena{
        estadosig=estado;
        
        //#llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let charcaracter= caracter(character);
        
        //#guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
        estado=tabla[estado][charcaracter];
    
      
        //#Si el valor obtenido es una E imprimimos cadena no valida
        if  estado=="E" {
            //println!("|     ",estadosig,"      |  ",character,"    |",simbolo," |     ",estado,"       |");
            body();
            println!("|              Cadena No Valida :(                   |
    +----------------------------------------------------+");
            }
        //contenido(estadosig,character,simbolo,estado);
    }

    //#al concluir si el estado no es 3 que es el de aceptacion imprimimos cadena no valida    
    if estado!="3" {
            println!("|              Cadena No Valida :(                   |
    +----------------------------------------------------+");
    }
    //si el estado es 3 es una cadena de aceptacion
    if estado=="3" {
        //println!("|     ",estado,"      |         |Fin Cadena |               |");
        body();
        println!("|                Cadena Valida                       |
    +----------------------------------------------------+");
    }
}
