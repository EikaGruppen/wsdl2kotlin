package inheritance.openclasses;

import dataclasses.Door;

import javax.xml.bind.annotation.XmlAccessType;
import javax.xml.bind.annotation.XmlAccessorType;
import javax.xml.bind.annotation.XmlSeeAlso;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.List;

@XmlAccessorType(XmlAccessType.FIELD)
@XmlSeeAlso({
        CLevel.class
})
public class BLevel extends ALevel {

    protected int b;
    protected BigDecimal bb;
    protected Door bbb;
    protected List<String> bbbb;
    protected boolean bbbbb;

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

    public Door getBbb() {
        return bbb;
    }

    public void setBbb(Door bbb) {
        this.bbb = bbb;
    }

    public List<String> getBbbb() {
        if (bbbb == null) {
            bbbb = new ArrayList<>();
        }
        return this.bbbb;
    }

    public boolean isBbbbb() {
        return bbbbb;
    }

    public void setBbbbb(Boolean bbbbb) {
        this.bbbbb = bbbbb;
    }
}
