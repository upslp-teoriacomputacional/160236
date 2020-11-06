use std::io;

fn main() {
    println!("Ingresa el palindromo: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    
    if is_palindrome(input.trim())
    {
        println!("La frase: {} , es palindromo",input.trim());
    }
    else
    {
        println!("La frase: {} , no es palindromo",input.trim());
    }
}

fn is_palindrome(frase: &str) -> bool {
    // aqui es donde se comprueban los indices
    frase.char_indices().filter(|&(_,carac)| carac.is_alphabetic())
    // se adjunta con la segunda prte de la cadena
    .zip(frase.char_indices()
    //para ponerlo en reversa
    .rev()
    //es un filtro para saber si es alfabetico
    .filter(|&(_,carac)| carac.is_alphabetic()))
    // comprueba hasta que los indices se crucen
    .take_while(|&((first_count, _), (last_count, _))| {first_count < last_count})
    //se checa todos los indices para ver si coinciden
    .all(|((_, first_char), (_, last_char))| {
        first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
    })
}