🧪 Caso 1
rust
fn main() {
let n = 10;
thread::spawn(|| {
println!("{n}");
});
}
¿Compila?  
¿Imprime?  
¿Error de lifetime?

🧪 Caso 2
rust
fn main() {
let n = 10;
thread::spawn(move || {
println!("{n}");
});
}
¿Compila?  
¿Imprime?  
¿Necesita join?

🧪 Caso 3
rust
fn main() {
let mut n = 10;
thread::spawn(move || {
n += 1;
});
println!("{n}");
}
¿Compila?  
¿Qué imprime el main?  
¿El thread modifica algo?

🧪 Caso 4
rust
fn main() {
let mut n = 10;
let h = thread::spawn(move || {
n += 1;
println!("thread: {n}");
});
h.join().unwrap();
println!("main: {n}");
}
¿Compila?  
¿Qué imprime?  
¿Por qué?

🧪 Caso 5
rust
fn main() {
let s = String::from("hola");
thread::spawn(|| {
println!("{s}");
});
}
¿Compila?  
¿Por qué?  
¿Qué pide Rust?

🧪 Caso 6
rust
fn main() {
let s = String::from("hola");
thread::spawn(move || {
println!("{s}");
});
}
¿Compila?  
¿Quién es dueño de s?  
¿El main puede usar s después?

🧪 Caso 7
rust
fn main() {
let n = 10;
thread::spawn(move || {
thread::sleep(Duration::from_millis(5));
println!("{n}");
});
println!("main");
}
¿Imprime siempre el thread?  
¿Puede no imprimirse?

🧪 Caso 8
rust
fn main() {
let n = 10;
thread::spawn(move || {
thread::yield_now();
println!("{n}");
});
}
¿Puede imprimir cortado?  
¿Puede no imprimir nada?

🧪 Caso 9
rust
fn main() {
let n = 10;
let r = &n;
thread::spawn(move || {
println!("{r}");
});
}
¿Compila?  
¿Por qué?  
¿Qué lifetime se viola?

🧪 Caso 10
rust
fn main() {
static X: i32 = 99;
thread::spawn(|| {
println!("{X}");
});
}
¿Compila?  
¿Por qué 'static acá sí funciona?