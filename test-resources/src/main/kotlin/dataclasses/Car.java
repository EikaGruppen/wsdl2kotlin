package dataclasses;

import javax.xml.bind.JAXBElement;
import javax.xml.bind.annotation.XmlElement;
import javax.xml.bind.annotation.XmlElementRef;
import java.util.ArrayList;
import java.util.List;

public class Car {
    @XmlElement(name = "requiredInteger")
    protected Integer requiredInteger;
    @XmlElement(name = "SCAused")
    protected Boolean requiredBoolean;
    @XmlElement(name = "nullableShort", nillable = true)
    protected Short nullableShort;
    @XmlElement(name = "listOfInternalClasses")
    protected List<Door> listOfInternalClasses;

    public List<Door> getListOfInternalClasses() {
        if (listOfInternalClasses == null) {
            listOfInternalClasses = new ArrayList<>();
        }
        return listOfInternalClasses;
    }

    public List<Door> getNullableListOfInternalClasses() {
        return nullableListOfInternalClasses;
    }

    @XmlElement(name = "nullableListOfInternalClasses", nillable = true)
    protected List<Door> nullableListOfInternalClasses;

    @XmlElementRef(name = "stringJAXBElement", type = JAXBElement.class, required = false)
    protected JAXBElement<String> stringJAXBElement;
    @XmlElementRef(name = "internalClassJAXBElement", type = JAXBElement.class, required = false)
    protected JAXBElement<Door> internalClassJAXBElement;

    @XmlElement(nillable = true)
    protected Towbar nullableInternalClass;

    protected int primitiveInt;
    protected boolean primitiveBoolean;
    protected long primitiveLong;
    protected byte[] primitiveByteArray;
    protected short primititveShort;
}
