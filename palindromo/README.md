En este codigo se va a resolver si una palabra es o no un palindromo, lo cual, se refiere a si se lee igual al derecho y al reves, por ejemplo oso o algo mas complejo como anita lava al tina, para ello se tienen las siguientes funciones claves del proyecto.

let mut input = String::new();
io::stdin().read_line(&mut input);    <------ se lee la cadena por teclado.

frase.char_indices().filter(|&(_,carac)| carac.is_alphabetic()) <-- aqui es donde se comprueban los indices, ademas se fija si es de un tipo caracter

.zip(frase.char_indices() <----- se adjunta con la segunda parte de la cadena

.rev() <----------------- se pone en sentido comtrario.

.filter(|&(_,carac)| carac.is_alphabetic())) <----------------- es un filtro para saber si es alfabetico 

.take_while(|&((first_count, _), (last_count, _))| {first_count < last_count}) <----------- comprueba hasta que los indices se crucen

.all(|((_, first_char), (_, last_char))| { 
    first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
}) <--------- se checa todos los indices para ver si coinciden
