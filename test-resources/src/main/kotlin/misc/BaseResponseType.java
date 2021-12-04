package misc.a;

import javax.xml.bind.annotation.XmlAccessType;
import javax.xml.bind.annotation.XmlAccessorType;
import javax.xml.bind.annotation.XmlElement;
import javax.xml.bind.annotation.XmlSeeAlso;
import javax.xml.bind.annotation.XmlType;
import misc.b.ResponseStateType;
import misc.c.CarCreateResponseType;


@XmlAccessorType(XmlAccessType.FIELD)
@XmlType(name = "baseResponseType", propOrder = {
    "responseState"
})
@XmlSeeAlso({
    CarCreateResponseType.class
})
public abstract class BaseResponseType {

    @XmlElement(name = "ResponseState", required = true)
    protected ResponseStateType responseState;

    /**
     * Gets the value of the responseState property.
     * 
     * @return
     *     possible object is
     *     {@link ResponseStateType }
     *     
     */
    public ResponseStateType getResponseState() {
        return responseState;
    }

    /**
     * Sets the value of the responseState property.
     * 
     * @param value
     *     allowed object is
     *     {@link ResponseStateType }
     *     
     */
    public void setResponseState(ResponseStateType value) {
        this.responseState = value;
    }

}
