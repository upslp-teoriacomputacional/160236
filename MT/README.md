Para este codigo se tomo como base el codigo proporcionado en python, y "traducido" a rust queda lo siguiente:


use std::process; <------ para registrar los procesos

fn mt (state: char, blank: char, in_tape: Vec<char>, fin: char, rules: Vec<Vec<char>>, position: usize) 

esta es la funcion principal, la cual va a recibir el estado, el simbolo en blanco de la cinta, la cinta de la mt, el estado final, las reglas y la posicion en la maquina en la que se encuentra. 

las variables y esta funcion son mutables debido a que se tienen que modificar de lo contrario el programa mostrara error.

 if tape.is_empty() <----- esto checa si la cinta esta vacia o no
    {
    	tape.push(blank);
    } 
    if pos < 0
    {
    	pos += tape.len()
    } 
    if pos >= tape.len() || pos < 0
    {
    	println!("Se inicializa mal la posicion");
    	process::exit(0x0100); <-------- esto terminara la aplicacion 
    } 
    
    while st != fin <-- este while se ejecutara hasta que se cumpla que se llego al final de la cinta
    {
        print! ("{}\t|",st); //impresion del estado en el que va , si es el cursos el que miestra lo hara resaldo, sino solo lo va a mostrar
        for i in 0..tape.len()
        {
             if i==pos 
             {
             	print! (" [{}]", tape[i]);
             }
             else
             {
             	print! (" {}", tape[i]);
             }
        }
        println!("");
        //se recorren las reglas creadas 
        //ayuda del alumno juan Herrera
        for rule in &rules
        {	
            if st == rule[0] && tape[pos] == rule[1]   
            {//Guarda cuando encuentra la regla correspondiente
            	v1 = rule[2];	//Caracter por el que se reemplaza
	            dr = rule[3];	//Dirección de la cinta
	            s1 = rule[4];	//Estado siguiente
            }
        }
        
         //Reglas de transición, maquina de Turing hecha por el alumno Juan Hernandez
         estas reglas son las que utilizara la maquina de turin
    rules = vec![
    			//q0
                vec! ['0','B','B','R','0'],
                vec! ['0','1','1','R','0'],
                vec! ['0','/','/','R','0'],
                vec! ['0','=','=','L','1'],
                //q1
                vec! ['1','1','x','L','2'],
                //q2
                vec! ['2','1','1','L','2'],
                vec! ['2','/','/','L','3'], 
                //q3
                vec! ['3','B','B','L','3'],
                vec! ['3','■','■','R','9'],
                vec! ['3','1','B','R','4'],
                //q4
                vec! ['4','x','x','R','4'],
                vec! ['4','B','B','R','4'],
                vec! ['4','/','/','R','B'],
                vec! ['4','=','=','R','5'],
                //q5
                vec! ['5','1','1','R','5'],
                vec! ['5','■','1','L','6'],
                //q6
                vec! ['6','=','=','L','6'], 
                vec! ['6','1','1','L','6'],
                vec! ['6','x','1','L','6'],
                vec! ['6','■','■','R','7'],
                vec! ['6','/','/','L','A'],
				//q7
                vec! ['7','=','=','R','7'], 
                vec! ['7','1','1','R','7'],
                vec! ['7','/','/','R','7'],
                vec! ['7','B','1','R','7'],
                vec! ['7','■','■','R','8'],
				//q8 (Estado final)

				//q9
                vec! ['9','/','/','R','9'],
                vec! ['9','1','1','R','9'],
                vec! ['9','x','1','R','9'], 
                vec! ['9','B','1','R','G'],
                vec! ['9','=','=','R','7'],
				//q10
                vec! ['A','B','B','L','A'],
                vec! ['A','1','1','R','0'],
                vec! ['A','■','■','R','7'],
				//q11
                vec! ['B','1','1','R','B'],
                vec! ['B','x','x','L','C'],
				//q12
                vec! ['C','B','B','R','C'],
                vec! ['C','/','/','L','D'],
                vec! ['C','1','x','L','2'],
				//q13
                vec! ['D','B','B','L','D'],
                vec! ['D','1','1','R','E'],
                vec! ['D','■','■','R','F'],
				//q14
                vec! ['E','B','B','R','E'],
                vec! ['E','x','x','R','E'],
                vec! ['E','/','/','R','E'],
                vec! ['E','=','=','R','5'],
				//q15
                vec! ['F','/','/','R','F'],
                vec! ['F','x','x','R','F'],
                vec! ['F','B','B','R','F'],
                vec! ['F','=','=','R','5'],
				//q16
                vec! ['G','=','=','R','G'], 
                vec! ['G','x','x','R','G'],
                vec! ['G','/','/','R','G'],
                vec! ['G','B','B','R','G'],
                vec! ['G','1','1','R','G'],
                vec! ['G','■','■','R','H'],
				//q17
                vec! ['H','1','1','R','H'],
                vec! ['H','■','1','L','I'],
				//q18
                vec! ['I','■','■','L','I'], 
                vec! ['I','x','x','L','I'],
                vec! ['I','1','1','L','I'],
                vec! ['I','=','=','L','I'],
                vec! ['I','/','/','L','J'],
				//q19
                vec! ['J','B','B','L','J'],
                vec! ['J','1','1','R','N'],
				//q20
                vec! ['K','/','/','R','K'],
                vec! ['K','x','x','R','K'],
                vec! ['K','=','=','R','K'],
                vec! ['K','B','B','R','K'], 
                vec! ['K','1','1','R','K'],
                vec! ['K','■','■','R','L'],
				//q21
                vec! ['L','1','1','R','L'],
                vec! ['L','■','1','L','I'],
				//q22
                vec! ['M','x','1','R','O'],
                vec! ['M','=','=','R','8'],
				//q23
                vec! ['N','/','/','R','M'],
                vec! ['N','B','1','R','K'],
				//q24
                vec! ['O','1','1','R','O'],
                vec! ['O','x','x','R','O'],
                vec! ['O','=','=','R','O'],
                vec! ['O','■','■','R','P'],
				//q25
                vec! ['P','1','1','R','P'],
                vec! ['P','■','■','L','Q'],
				//q26
                vec! ['Q','1','■','L','R'],
				//q27
                vec! ['R','■','■','L','R'],
                vec! ['R','1','1','L','R'],
                vec! ['R','=','=','L','S'],
				//q28
                vec! ['S','x','x','L','S'],
                vec! ['S','1','1','R','M']
            ];
            
            
