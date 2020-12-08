/*
Proyecto Final en rust 
name and surname Alexis Antonio Balderas Aguirre
major: 160236 
speciality professor: Juan Carlos González Ibarra
institution: Universidad Politécnica de SLP
enrollment: ITI
*/

use std::io;
fn mt (floor: Vec<i32>, rules: Vec<Vec<i32>>, inicio: i32)
{
    let mut check: Vec<i32>; //los pisos que entran
    let mut fin: Vec<i32> = Vec::new(); //el orden final
    check = floor; //se copia los pisos que se ingresaron
    let mut pisoA = inicio; //piso actual
    let mut pisoD: i32; //piso destino
    pisoD = 10; //se pone un piso destino para que no encuentre errores;
    let mut aux: i32 = 0; //auxiliar
    let mut borrar: i32 = 0; //que elemento va a borrar
    let mut heuristic = 10; //peso
    let mut lugar: i32 = 0; //que piso se esta comprobando
    fin.push(pisoA); //ingresa el piso de inicio como el primero que visita
    while !check.is_empty() //se repetira siempre y cuando el vector a checar tenga datos
    {
        aux = 0;
        borrar = 0;
        heuristic = 10;
        lugar = 0; //se inicializan los valores en un valor neutro que ira cambiando
        if check.len() > 1 //comprueba si hay mas de 1
        {
            for x in 0..check.len() //recorre todo el vector 
            {
                lugar = check[x]; //agarra el piso en el que va
                if heuristic > rules[pisoA as usize][lugar as usize] && rules[pisoA as usize][lugar as usize] != 0 //comprueba el peso dado en el grafo y que no se va a ciclar en visitar el mismo piso
                {
                    println!("vector: {:?}", check);
                    println!("x: {:?}", x);
                    pisoD = check[x];
                    borrar = aux;
                    heuristic = rules[pisoA as usize][lugar as usize];
                    println!("regla: {:?}", rules[pisoA as usize][lugar as usize]);
                    println!("lugar: {:?}", lugar);
                    println!("peso: {:?}", heuristic);
                    println!("pisoA: {:?}", pisoA);
                    println!("pisoD: {:?}", pisoD);
                    println!("------------------------");
                }
                aux = aux + 1;
            }
            pisoA = pisoD; //ahora el piso a visitar es el piso actual
            fin.push(pisoD); //ingresa el piso destino al vector que muestra el final
            check.remove(borrar as usize); //elimina el valor del vector a checar
        }
        else //si no hay mas de uno es porque es el ultimo a visitar y es el que se mete
        {
            fin.push(check[0]); //ingresa el unico valor del vector
            check.remove(0 as usize); //remueve el unico valor restante del vector
        }
        
    }
    println!("El orden es: {:?}", fin); //se imprime el orden
}

fn main() {
    let _rules: Vec<Vec<i32>>; //se declara el grafo de las reglas
    let mut comprobar: Vec<i32> = Vec::new(); //vector para tener los pisos
    _rules = vec![ //grafo de pisos
        vec! [0, 1, 2, 3, 4],//PB
        vec! [1, 0, 1, 2, 3],//P1
        vec! [2, 1, 0, 1, 2],//P2
        vec! [3, 2, 1, 0, 1],//P3
        vec! [4, 3, 2, 1, 0],//P4
    ];
    println!("Ingresa el orden: ");
    let mut input = String::new(); //se lee la cadena
    io::stdin().read_line(&mut input);
	for w_chars in input.chars() { //for que cambia de una cadena a un vector lleno de enteros
        if w_chars == '0'
        {
            comprobar.push(0);
        }
        else if w_chars == '1'
        {
            comprobar.push(1);
        }
        else if w_chars == '2'
        {
            comprobar.push(2);
        }
        else if w_chars == '3' 
        {
            comprobar.push(3);
        }
        else if w_chars == '4' 
        {
            comprobar.push(4);
        }
    }
    println!("Ingresa el piso de origen: ");
    io::stdin().read_line(&mut input); //se lee el piso de inicio
    let mut empezar: i32 = 0;
    for w_chars in input.chars() {
        if w_chars == '0'
        {
            empezar = 0;
        }
        else if w_chars == '1'
        {
            empezar = 1;
        }
        else if w_chars == '2'
        {
            empezar = 2;
        }
        else if w_chars == '3' 
        {
            empezar = 3;
        }
        else if w_chars == '4' 
        {
            empezar = 4;
        }
    }
    mt(comprobar,_rules,empezar); //se manda a llamar al funcion
}