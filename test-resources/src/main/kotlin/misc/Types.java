package misc.a;

public class Types {
    @XmlElement(name = "Integer")
    protected Integer integer;
    @XmlElement(name = "Cars")
    protected List<Car> cars;
    @XmlElement(name = "Bool")
    protected Boolean bool;
    @XmlElement(name = "NillableShort", nillable = true)
    protected Short nillableShort;

	@XmlElement(nillable = true)
    protected Car car;

    protected int primInt;
    protected boolean primBool;
	protected long primLong;
    protected byte[] primByteArray;
    protected short primShort;
    protected double primDouble;
	protected float primFloat;
}
