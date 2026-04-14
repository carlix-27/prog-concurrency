use std::thread;

fn main(){
    hello()
}

fn hello() {
    let n = 10;
    let mut m = 10;

    thread::scope(|s| {
        s.spawn(|| {
            m = m + 1;
            println!("Hello from thread 1, n = {n}");
        });
      s.spawn(|| println!("Hello from thread 2, n = {n}"));
    });
    println!("{m}");
}

/*

Como n no muta puedo hacer copy por cada thread.

*/
// Los dos threads estan imprimiendo una variable inmutable, como nadie la cambia, no importa.
// Que pasa solo con n?
// Solo le estoy pasando la referencia.
// como es un int, podria copiarse. En gral cuando laburamos dentro del scope se pasa una referencia.
// se puede copiar porque tiene un size fijo
// si el valor vive en el heap, tenemos que pasarle la referencia para referir el heap.

/*
Si sobre eso hago algo:
Tiene que seguir con el scope.
Esta esperando al join. Lo que esta haciendo al final esta haciendo un join, es un "superjoin"
La linea 18 no se ejecuta hasta que todas las threads dentro del scope.

Cuando hacemos que sea mutable, necesariamente es una referencia.

Para que se puede modificar necesariamente debo pasarle la referencia.

Por que no pasa nada?
Pasamos solo UNA REFERENCIA MUTABLE.




Lo revisa tanto a nuestro scope, como a nivel de otros threads.
Los threads funcionan como "scopes". Yo no puedo pasar multiples referencias mutables a distintos scopes
De la misma forma pasa con los threads.

Si es mutable no puede pasar una refernecia.

Mutarlo

Hace m mutable, lo modifica. Es un "borrow mutable"

No podemos tener multipls referencias mutables a una variable.

for i in 0 ..2 {
      s.spawn(|| {
            m = m + 1; // Esto esta mal
            println!("Hello from thread 1, n = {n}");
       });
}


- thread sigue la misma logica que las referencias.
- No podemos inmutable y mutable al mismo tiempo
- esta mal porque no podemos tener dos referencias mutables
- las reglas de las referencias aplican aca tambien!


___________

Tema parcial:
- Codigo no rust perfecto
- conceptualmente no puede faltar lo que vamos a hacer ahora
- ejercicio con bastante codigo
- parte pesada de codigo va a estar en rust. Producir codigo de Rust, que las ideas sean claras.
- lo que vimos recien que esten plasmadas, abstracciones que vimos
- ej: simulacion, varios threads compitiendo por recursos
- resolve el problema de los filosofos, podria ser un ejercicio perfectamente
- cual es cada thread
- que representa una thread. dentro del contexto del problema. Que es esa thread en mi problema?
- como traslado con threads, que representa una thread y eventualmente como hago esa simulacion con las primitivas.
- no errores con mutex (conceptualmente debe estar correcto).
- snippets de java puede ser que pasa que problema
- V o F
- problema mas pesado (40 puntos)

- entender para saber que existe (tema java)

- Podemos buscar ejercicios
- gpt algunos puede modelar
- investigar con esto
- preferiblemente tener mejor conceptos bien atrapados
- la parte de codigo va a ser un desafio mas complejo
- conceptos propios de threads


*/

fn hello2(){
    let n = 10;
    let mut m = 10;

    // Nunca mas va a leerse el resultado de m. Ese valor, nunca ladie mas lo lee.
    // La modificacion que hago, no tuvo efecto sobre m
    // Lo podes seguir leyendo porque es Copy.
    // Si fuera string que va al heap, esto no funcionaria, porque no puedo hacer un copy.
    thread::spawn(move || {
        m = m + 1;
        println!("Hello from thread 1, n = {n}"); // El valor cuando lo quiere leer, termino la thread principal.
    });
    println!("{m}");
    /*
    Como corre primero el m, particularmente para ese thread, el valor de n no existe, ya que el thread ppal termino cuando imprimio.

    El move lo que hace es mover las referencias.
    Los clojures. Son lambdas que tienen alguna particularidad mas, si modifican algo tienen que poder tener acceso a eso
    en el caso de un thread incluso es un poco mas, cuando hago un move se copian todas las referencias ese ownership se pasa.

    move -> todo lo que se use como referencia ya sea mutable e inmutable
    Transfiere ownership.

    La primer implicancia es que el thread ppal no puede usar ni n ni m.
    Por que no hace como hago el scope?
    Que diferencia hay?
    - lifetimes de las variables
    - que es justamente lo que rompimos recien. AL hacer un move y al hacer un print sobre eso, se pasa la referencia.

    Cuando hago el move, paso referencias de todo.
    Mueve referencias, no pasa el ownership.
    El move es por referencia, por eso puedo usar luego el print m pero no se que va a pasar.

    Es que lo que te genera son referencias.

    M no podria imprimirlo porque en teoria el main perdio ownership.


   move para que sea owner de eso. Move mueve el ownership, si hacemos un move dentro de un metodo no podemos seguir referenciarlo

   Si es referencia mutable o inmutable, pasalo a valor.

Cuando pasas un move, se pasa el ownership.

Si hace un move por copy, porque no hace acceso a n.
Por que pasa esto?
10
Hello from thread 1, n =


Que hacia el sleep?

Hello 3 es el mas claro y es lo que deberia de pasar:




    */

}