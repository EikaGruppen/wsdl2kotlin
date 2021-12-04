package misc.a;

public interface CarsService {

    public GetCarsResponse getCars(

        @WebParam(partName = "SoapHeader", name = "AutHeader", targetNamespace = "http://asdf.com", header = true)
        misc.b.HeaderType soapHeader,
        @WebParam(partName = "GetCars", name = "GetCars", targetNamespace = "http://asdf.com")
        GetCars getCars
    );
}
