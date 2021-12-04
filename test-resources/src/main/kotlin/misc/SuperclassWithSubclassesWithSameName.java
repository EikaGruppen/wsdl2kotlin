package misc.a;

import javax.xml.bind.annotation.XmlSeeAlso;

@XmlSeeAlso({
    misc.b.Subclass.class,
    misc.c.Subclass.class
})
public class SuperclassWithSubclassesWithSameName {

}
