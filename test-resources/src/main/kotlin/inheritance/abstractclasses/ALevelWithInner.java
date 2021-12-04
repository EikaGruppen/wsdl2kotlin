package inheritance.abstractclasses;

public abstract class ALevelWithInner {

    protected int a;
    protected ALevelWithInner.Inner inner;

    public int getA() {
        return a;
    }

    public void setA(int a) {
        this.a = a;
    }

    public Inner getInner() {
        return inner;
    }

    public void setInner(Inner inner) {
        this.inner = inner;
    }

    public static class Inner {
        protected String ia;

        public String getIa() {
            return ia;
        }

        public void setIa(String ia) {
            this.ia = ia;
        }
    }
}
