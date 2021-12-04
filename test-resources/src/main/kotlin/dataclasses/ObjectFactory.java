package dataclasses;

import javax.xml.bind.JAXBElement;
import javax.xml.namespace.QName;

public class ObjectFactory {

    public JAXBElement<String> createStringJAXBElement(String value) {
        return new JAXBElement<String>(new QName(""), String.class, Integer.class, value);
    }

    public JAXBElement<Door> createInternalClassJAXBElement(Door value) {
        return new JAXBElement<Door>(new QName(""), Door.class, Integer.class, value);
    }
}
