package java;

public class TestClass1 {

    // Static field (class variable)
    public static int staticCounter = 0;

    // Instance fields (object variables)
    private int id;
    protected String name;

    // Constructor (direct method)
    public TestClass1(int id, String name) {
        this.id = id;
        this.name = name;
    }

    // Static method (does not depend on an instance)
    public static void incrementCounter() {
        staticCounter++;
    }

    // Instance method (virtual method if not private/final)
    public void displayInfo() {
        System.out.println("ID: " + id + ", Name: " + name);
    }

    // Private method (direct method)
    private void logInternal() {
        System.out.println("Internal log for ID: " + id);
    }

    // Final method (still virtual but not overridable)
    public final String getName() {
        return name;
    }

    // Protected method (virtual method)
    protected void resetName() {
        this.name = "Unnamed";
    }
}
