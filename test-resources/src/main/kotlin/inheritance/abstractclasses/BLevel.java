package inheritance.abstractclasses;

import java.math.BigDecimal;

public abstract class BLevel extends ALevel {

    protected int b;
    protected BigDecimal bb;

    public int getB() {
        return b;
    }

    public void setB(int b) {
        this.b = b;
    }

    public BigDecimal getBb() {
        return bb;
    }

    public void setBb(BigDecimal bb) {
        this.bb = bb;
    }
}
