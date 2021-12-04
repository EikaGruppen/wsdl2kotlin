package misc.a;

import misc.b.Car;

@XmlSeeAlso({
    Car.class,
    Bus.class
})
public class VehicleType {
    protected int doors;
}
