/*
RE en rust 
name and surname Alexis Antonio Balderas Aguirre
major: 160236 
speciality professor: Juan Carlos González Ibarra
institution: Universidad Politécnica de SLP
enrollment: ITI
*/

//Se usa para las expresiones regulares
 extern crate regex;		
 use regex::Regex;
                                  
 fn main()
 {
     //se definen los vectores
     let mut tokens: Vec<String> = Vec::new();
     //transforma de una cadena a una lista de palabras
     let  source_code = "int result = 1;".split(" "); 
     //se definen los tipos de valores
     let types: [&str; 3] = ["str", "int", "bool"];
    //se crean las expresiones regulares
     let digito = Regex::new(r"[0-9]").unwrap();
     let letras1 = Regex::new(r"[a-z]").unwrap();
     let letras2 = Regex::new(r"[A-Z]").unwrap();
     let operador = Regex::new(r"(\+|\-|\*/)").unwrap();
 
 
 
     //se recorre toda la cadena dada
     for word in source_code
     {
         let mut aux_tokens: String = "".to_string();
         // checa si es un tipo declarado
         if types.contains(&word)
         {
             aux_tokens = format!("{}{}","DATATYPE: ",word);
         } // busca en los identificadores que tipo es
         else if letras1.is_match(word) || letras2.is_match(word)
         {
             aux_tokens = format!("{}{}","IDENTIFIER: ",word);
         }// checa que operador es
         else if operador.is_match(word) 
         {
             aux_tokens = format!("{}{}","OPERATOR: ",word);
         }
         // This will look for integer items and cast them as a number
         else if digito.is_match(word)
         {
             //Teke the ; from the string
             let aux_w: u8 = word.as_bytes()[(word.len() - 1) as usize];
             let mut aux_w2: String = word.to_string();
 
             if (aux_w as char) == ';'
             {
                 aux_w2.remove(word.len() - 1);
                 aux_tokens = format!("{}{}","INTEGER ", aux_w2);
                 tokens.push(aux_tokens);
                 aux_tokens = format!("{}{}","END_STATEMENT",";");
             }
             else
             {
                 aux_tokens = format!("{}{}","INTEGER ",word);
             }
         }
 
         if aux_tokens != ""
         {
             tokens.push(aux_tokens);
         }
     }
 
     for token in &tokens
     {
         println!("{}", token);// Outputs the token array
     }
 
 }
 
 fn variable_prolog(w_string: String)->bool
 {
     let mut w: Vec<char> = Vec::new();
 
     for w_chars in w_string.chars() {
         w.push(w_chars);
     }
 
     let aux_w: Vec<_> = w[0].to_uppercase().collect();
 
     //(str)-->bool. True si "w" es un nombre de variable correcto'''
     if w[0].is_alphabetic() && w[0] == (aux_w[0] as char) || w[0] == '_'
     {
         //El primer caracter es una mayuscula o un subrayado
         //Se quita el primer caracter
         w.reverse();
         w.pop();
         w.reverse();
         while !w.is_empty() && (w[0].is_numeric() || w[0]  ==  '_')
         {
             //Mientras queden caracteres en "w" y el primer caracter actual sea un alfanumerico o un subrayado, todo esta bien
             //Quitar el primer caracter
             w.reverse();
             w.pop();
             w.reverse(); 
             if w.is_empty()
             {
                   //Si ya no quedan elementos a revisar, es una variable PROLOG
                 return true;
             }
         }                       
     }
     
     return false;
 }