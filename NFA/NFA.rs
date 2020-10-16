/*
NFA en rust 
name and surname Alexis Antonio Balderas Aguirre
major: 160236 
speciality professor: Juan Carlos González Ibarra
institution: Universidad Politécnica de SLP
enrollment: ITI
*/

//libreria para las expresiones rgulares
extern crate regex;

//se llaman las librerias
use std::io;        //para los inputs
use regex::Regex;   //para las expresiones regulares
use std::process;   //para los procesos

//se verifican los caracteres de entrada
//comienza la ayuda del alumno Humberto Herrera
//<------------------------------------------>


fn caracter(character: char, estado:i32) -> i32
{
    let fin="\r";
    let a = Regex::new(r"a").unwrap();  //Expresión regular para a*
    let b = Regex::new(r"b").unwrap();  //Expresión regular para b
    
    //Pasar caracter a una cadena
    let mut s: String = "".to_string();
	s.push(character);
	let new_character: &str = &s[..];

    //Si se recibe cualquier estado correspondiente al vacío devielve 0
    if estado == 0 || estado == 1 || estado == 3 || estado == 4 || estado == 6 || estado == 7 || estado == 9 || estado == 10 
    {
        return 0;
    } 

     //Si recibe el estado final muestra que la cadena es valida automáticamente
    if estado == 11
    {
        println! ("|      {}      |         |Fin Cadena |               |",estado);
        body();
        println! ("|                Cadena Valida  :D                   |");
        println! ("+----------------------------------------------------+");
        process::exit(0x0100);
    }

    //comparamos si la entrada es a o b
    if a.is_match(new_character)
    {
        return 1;       //a
    }
    else 
    {
	    if b.is_match(new_character) 
	    {
	        return 2;       //b
	    }
	    else 
	    {
	    	if new_character==fin
		    {
		    	return 3;       //Fin de cadena
	    	}
	    }
	    //si no es ni una a ni una b entonces es un caracter no valido
		println!("Error el caracter: {} no es valido",character);
		process::exit(0x0100);
    }    
}

//<-------------------------------------->
//fin de ayuda del alumno Humberto Herrera

//encabezado de la tabla
fn encabezado()
{
    println! ("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
    body();
}

//se guardan los valores cuando se encontran en el ciclo
fn contenido(estadosig: i32,character: char,simbolo: &str,estado: i32)
{
    println! ("|      {}       |    {}    | {}|       {}       |",estadosig,character,simbolo,estado);
    body();
}

fn body()
{
    println! ("+--------------+---------+-----------+---------------+");
}


fn main()
{
    //Tabla de estados del autoamta
    let tabla: Vec<Vec<char>>;
    tabla = vec![
                vec! ['1','E','E','E'],     // q0
                vec! ['2','E','E','E'],     // q1
                vec! ['E','3','E','E'],     // q2
                vec! ['4','E','E','E'],     // q3
                vec! ['5','E','E','E'],     // q4
                vec! ['E','E','6','E'],     // q5
                vec! ['7','E','E','E'],     // q6
                vec! ['8','E','E','E'],     // q7
                vec! ['E','9','E','E'],     // q8
                vec! ['a','E','E','E'],     // q9
                vec! ['b','E','E','E'],     // q10
                vec! ['E','E','E','A']      // q11
            ];
    
    //estado inical
    let mut estado: i32 = 0;
    let mut simbolo: String = "".to_string();
    println! ("|    Ingrese una cadena a evaluar:    |");
    let mut cadena = String::new();		
    io::stdin().read_line(&mut cadena);
    body();
    encabezado();

    //aqui se recorrer la cadena
    for  character in cadena.chars()
    {
        //se comprueba que el caracter sea un caracter valido
        let mut charcaracter = caracter(character,estado); //funcion definida arriba

        while charcaracter == 0
        {
            let  estadosig: i32 = estado;
            //guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
            estado =  (tabla[ estado as usize][charcaracter as usize]) as i32 - '0' as i32 ;

            /////////////Para la recurción en caso de que halla vacío ante o después de b/////////////
            
            if estadosig == 1  && character == 'b'  
            {
                estado = 4;                         
            }

            if estadosig == 7  && character == '\r'
            {
                estado = 10;                        
            }

            //empieza a* recursiva 
            if estadosig == 4  && character != 'b' 
            {
                estado = 1;                       
            }

            if estadosig == 10  && character != '\r'
            {
                estado = 7;                         
            }
            

            if(estado == 49)
            {   
                estado=10;      
            }
            
            if(estado == 50)
            {
                estado=11;      
            }

            //muestra el vacío
            contenido(estadosig,' ',&"          ".to_string(),estado);

            //Vuelve a calcular charcaracter
            charcaracter = caracter(character,estado); //funcion definida arriba
        }

        // Se calcula el estado de nuevo
        let  estadosig: i32 = estado;
        estado = (tabla[ estado as usize][charcaracter as usize]) as i32 - '0' as i32 ;

        //Identifica los smibolos a, b o F
        if charcaracter == 1    
        {
            simbolo = "    a     ".to_string();
        }
        else if charcaracter == 2
        {
            simbolo = "    b     ".to_string();
        }
        else if charcaracter == 3
        {
            simbolo = "Fin Cadena".to_string();
        }

        //Si es error
        if estado==21
        {
            let mut character_imp = character;
            if character_imp == '\r'
            {
                character_imp = ' ';
            }

            println! ("|      {}       |    {}    |{} |     Error     |",estadosig,character_imp,simbolo);
            body();
            println! ("|              Cadena No Valida :(                   |");
            println! ("+----------------------------------------------------+");
            process::exit(0x0100);
        }
        contenido(estadosig,character,&simbolo,estado); 
    }  
}