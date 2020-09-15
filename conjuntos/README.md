# 160236
En este codigo escrito en rust se presenta lo siguiente:

let es para definir una variable

let mut es para definir una variable que mas adelante puede cambiar,

.contains(&) nos definira la pertenencia, es decir, nos dira si se encuentra o no el elemento que le digamos.

HashSet<_> = a.iter().collect(); convertira (en este caso) de un vector o un array a un hashset.

.remove(&) va a remover el elemento que se le indique

.clear() va a limpiar el hashset, es importante no confundirlo con remove.

.clone() servira para clonar en una nueva variable.

A.intersection(&B) nos indicara la interseccion que estos tengan, es decir, los elementos que compartan.

A.difference(&B)) mostrara los elementos que estos no compartan.

A.symmetric_difference(&B) mostrara si existe o no una diferencia simetrica.

B.is_subset(&A)) nos indicara si b un subconjunto de a.

B.is_superset(&A)) nos indicara si b un superconjunto de a.

