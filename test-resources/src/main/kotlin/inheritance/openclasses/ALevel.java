package inheritance.openclasses;

import javax.xml.bind.JAXBElement;
import javax.xml.bind.annotation.XmlAccessType;
import javax.xml.bind.annotation.XmlAccessorType;
import javax.xml.bind.annotation.XmlElementRef;
import javax.xml.bind.annotation.XmlSeeAlso;

@XmlAccessorType(XmlAccessType.FIELD)
@XmlSeeAlso({
        BLevel.class
})
public class ALevel {

    protected int a;
    @XmlElementRef(name = "aa", type = JAXBElement.class, required = false)
    protected JAXBElement<Window> aa;

    public int getA() {
        return a;
    }

    public void setA(int a) {
        this.a = a;
    }

    public JAXBElement<Window> getAa() {
        return aa;
    }

    public void setAa(JAXBElement<Window> aa) {
        this.aa = aa;
    }
}
