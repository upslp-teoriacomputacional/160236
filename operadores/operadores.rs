/*
Operadores en rust 
name and surname Alexis Antonio Balderas Aguirre
major: 160236 
speciality professor: Juan Carlos González Ibarra
institution: Universidad Politécnica de SLP
enrollment: ITI

*/
    fn main(){
        //se llaman a las funciones
        funcionOr();
        println!("---------------");
        funcionAnd();
        println!("---------------");
        funcionNot();
        println!("---------------");
        funcionY();
        
    }
    
    //en esta funcion se comprobara el operador logico or
    fn funcionOr(){
        let booleanos = [false, true]; //este es el arreglo que se define los valores booleanos mas no strings
         println!("x\ty\tx or y");
         for x in &booleanos{ //se recorre el arreglo con dos foreach
               for y in &booleanos{
                    print!("{}",x);  print!("\t{}", y);  println!("\t{}\n",x|y); /*el operador logico | (or) informara al programa si alguno 
                     es verdadero y la salida sera verdadero*/
               }
                
         }
    }
    //en esta funcion se comprobara el operador logico and
    fn funcionAnd(){
        let booleanos = [false, true]; //este es el arreglo que se define los valores booleanos mas no strings
         println!("x\ty\tx and y");
         for x in &booleanos{
               for y in &booleanos{
                    print!("{}",x);  print!("\t{}", y);  println!("\t{}\n",x&y); /*el operador logico & (and) informara al programa si ambos 
                    son verdadero y la salida sera verdadero*/
               }
                
         }
    }
    //en esta funcion se comprobara el operador logico not
    fn funcionNot(){
        let booleanos = [false, true]; //este es el arreglo que se define los valores booleanos mas no strings
         println!("x\tnot x");
         for x in &booleanos{
             print!("{}",x);  println!("\t{}\n",!x); /*el operador logico ! (not) informara al programa si alguno 
             es verdadero y la salida sera verdadero*/
         }
    }
    
    //en esta funcion se comprobara el operador logico o exclusivo
    fn funcionY(){
        let booleanos = [false, true]; //este es el arreglo que se define los valores booleanos mas no strings
         println!("x\ty\tx and y");
         for x in &booleanos{
               for y in &booleanos{
                    print!("{}",x);  print!("\t{}", y);  println!("\t{}\n",x^y); /*el operador logico ^ (o exclusivo) informara al programa si alguno 
             es verdadero y la salida sera verdadero unicamente si y solo si uno es verdadero y no ambos*/
               }
                
         }
    }
