/*
Conjuntos en rust 
name and surname Alexis Antonio Balderas Aguirre
major: 160236 
speciality professor: Juan Carlos González Ibarra
institution: Universidad Politécnica de SLP
enrollment: ITI

*/
use std::collections::HashSet;
    fn main(){
        //se llaman a las funciones
        pertenencia();
        transformarConj();
        quitar();
        clearSet();
        copiar();
        diferencia();
        simetrica();
        subconjunto();
        superconjunto();
    }
    //let mut c = HashSet::new();
    
    fn pertenencia(){
        let a: HashSet<_> = [1,2,3,4,5].iter().cloned().collect(); //se definen el set
        a.contains(&1); //se checa si contiene o no los datos
        !a.contains(&1);
        a.contains(&10);
        !a.contains(&10);
    }
    
    //Convertir a un conjunto
    fn transformarConj(){
        let a = vec![1,2,3,4,5]; //se define el vector
        let conjuntoA : HashSet<_> = a.iter().collect(); //se creo el conjunto
        println!("The set is: {:?}", &conjuntoA);
        
        let b = [1,2,3,4,5]; //se define el arreglo
        let conjuntoB: HashSet<_> = b.iter().collect(); //se creo el conjunto
        println!("The set is: {:?}", &conjuntoB);
        
       // let c = "Hola mundo";
       // let conjuntoC : HashSet<_> = c;
     //   println!("The set C is : {:?}", &conjuntoC);
    }
    
    fn quitar()
    {
        let mut A: HashSet<_> = [0,1,2,3,4,5].iter().collect();
        A.remove(&2); //elimina el numero 2
        println!("The set after to delete: {:?}", &A);
    }
    
    fn clearSet(){
         let mut A: HashSet<_> = [0,1,2,3,4,5].iter().collect();
         A.clear(); //limpia el set
         println!("The set clear: {:?}", &A);
    }
    //copiar un set a otro
    fn copiar(){
        let mut A: HashSet<_> = [0,1,2,3,4,5].iter().collect();//se definen el set
        let mut B: HashSet<_> = A.clone(); //copia un set a otro
        println!("The set Ar: {:?}", &A);
        println!("compare set B: {:?}", &B);
        
        //insertar 
        
         B.insert(&2); //inserta en el set
         println!("The new set B: {:?}", &B);
    }
    
    //interseccion
    fn intereseccion(){
        let mut A: HashSet<_> = [1,2,3,4,5].iter().collect();//se definen los sets
        let mut B: HashSet<_> = [3,4,5,6,7].iter().collect();
        println!("The intersection {:?}= ", A.intersection(&B));
    }   

    //Diferencia
    fn diferencia(){
        let mut A: HashSet<_> = [1,2,3,4,5].iter().collect(); //se definen los sets
        let mut B: HashSet<_> = [3,4,5,6,7].iter().collect(); //checa el resto de los numeros diferentes
        println!("The difference {:?}= ", A.difference(&B));
    }

    //Diferencia simetrica
    fn simetrica(){
        let mut A: HashSet<_> = [1,2,3,4,5].iter().collect(); //se definen los sets
        let mut B: HashSet<_> = [3,4,5,6,7].iter().collect();
        let mut C: HashSet<_> = [].iter().collect(); //asi se declara un set vacio
        println!("The symmetric_difference = {:?}", A.symmetric_difference(&B));
        println!("The symmetric_difference = {:?}", B.symmetric_difference(&A));
        println!("The symmetric_difference = {:?}", A.symmetric_difference(&C));
        println!("The symmetric_difference = {:?}", B.symmetric_difference(&C));
    }

    //Subconjunto
    fn subconjunto(){
        let mut A: HashSet<_> = [1,2,3,4,5].iter().collect();//se definen los sets
        let mut B: HashSet<_> = [3,4,5,6,7].iter().collect();
        println!("The subset = {} ",A.is_subset(&B)); //se sacan los subconjuntos
        println!("The subset = {} ",B.is_subset(&A));
    }

    //Superconjunto
    fn superconjunto(){
        let mut A: HashSet<_> = [1,2,3,4,5].iter().collect();//se definen los sets
        let mut B: HashSet<_> = [3,4,5,6,7].iter().collect();
        println!("The superset = {} ",B.is_superset(&A)); //se sacan los superconjuntos
        println!("The superset = {} ",A.is_superset(&B));
    }