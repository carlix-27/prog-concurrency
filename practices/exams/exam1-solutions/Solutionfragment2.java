import java.util.concurrent.atomic.AtomicInteger;

class MaxHolder {
    private AtomicInteger max = new AtomicInteger(0);
    void updateIfGreater(int value) {
        if (value > max.get()) { max.set(value);}
    }

    int getMax() { return max.get(); }
}




// Explicacion teorica y correccion concreta de por que esta mal
/*



*/


// Solucion

class Solutionfragment2 {
    
}
