package inheritance.abstractclasses;

import javax.xml.bind.annotation.XmlSeeAlso;

@XmlSeeAlso({
        BLevel.class
})
public abstract class ALevel {

    protected int a;

    public int getA() {
        return a;
    }

    public void setA(int a) {
        this.a = a;
    }
}
