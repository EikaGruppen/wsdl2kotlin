package inheritance.openclasses;

import javax.xml.bind.JAXBElement;
import javax.xml.namespace.QName;

public class ObjectFactory {

    public JAXBElement<Window> createAa(Window value) {
        return new JAXBElement<Window>(new QName(""), Window.class, Integer.class, value);
    }

}
