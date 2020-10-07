El siguiente documento fue hecho en : https://paiza.io/es

Se tuvieron errores al intentar compilar en VS code y el compilador anterior no detecta las entradas por teclado


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
        
        en el anterior fragmento lo que se realiza es que se compara para saber si es el fin, algun digito o bien un caracter
        
        En el main, se definen las tablasy las cadenas que se usaran para realizar la busqueda o la insercion
        
        Despues se va a comprobar si es valido o no el caracter (con la funcion antes descrita) y se dara una accion segun sea valido
