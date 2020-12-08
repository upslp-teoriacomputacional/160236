Proyecto final en RUST

El proyecto que se tomo fue una creación de un elevador integilente, el cual determinaria la manera mas optima de recorrer las entradas.

para ello usamos lo siguiente:

use std::io; <-------- se uso la libreria para leer por teclado

let _rules: Vec<Vec<i32>>; <--- se declara el grafo de las reglas
    _rules = vec![ //grafo de pisos
        vec! [0, 1, 2, 3, 4],//PB
        vec! [1, 0, 1, 2, 3],//P1
        vec! [2, 1, 0, 1, 2],//P2
        vec! [3, 2, 1, 0, 1],//P3
        vec! [4, 3, 2, 1, 0],//P4
    ];
                                este grafo nos ayuda a conocer los pesos que se tienen en las trancisiones

 io::stdin().read_line(&mut input); <----------- de esta manera se lee una entrada de teclado, los cuales seran los pisos y el piso de origen.
 
 for w_chars in input.chars() {} en este for lo que se hace es es checar la entrada (dada en un string) y se va creando un vector dado en enteros.
 
 fn mt (floor: Vec<i32>, rules: Vec<Vec<i32>>, inicio: i32) {} es la funcion principal.
  
 a este se le mandan, los pisos, el grafo que se establecio al principio y el piso de inicio.
 
  let mut fin: Vec<i32> = Vec::new();  <---- se declara el vector que tendra el orden final
 
 fin.push(pisoA); <--- se ingresa el piso de inicio como el primero que visita, esto se hace con el metodo de vector push
 
 while !check.is_empty() //se repetira siempre y cuando el vector a checar tenga datos {} <----- este while se ejecutara siempre y cuando check (= floor) no este vacio.
 
 if check.len() > 1 {} <------ se comprueba si hay mas de 1
 
for x in 0..check.len() //recorre todo el vector {} este for recorrera todo check (= floor) hasta que se finalice y guarda el valor que pese menos, esto se hace para ir metiendo el que sea mas optimo al viajar.

fin.push(pisoD); <--- ingresa el piso destino al vector que muestra el final
check.remove(borrar as usize); <----- elimina el valor del vector a checar

else {} <----------- si no hay mas de 1 y evitar que entre en panico el programa, se elimina el unico valor y se guarda en el vector de fin.

println!("El orden es: {:?}", fin); <----------- se imprime el orden optimo
