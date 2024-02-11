package Fejvm;
import java.io.Serializable;

public class hi implements Cloneable, Serializable {
    private final double real;
    private final double imag;

    public hi(double real) {
        this.real = real;
        this.imag = 0;
    }

    public hi(double real, double imag) {
        this.real = real;
        this.imag = imag;
    }

    public double getReal() {
        return this.real;
    }

    public double getImag() {
        return this.imag;
    }

    public double abs() {
        return Math.sqrt(this.real * this.real + this.imag * this.imag);
    }
}
