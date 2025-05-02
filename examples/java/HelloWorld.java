public class HelloWorld {

    public static void main(String[] args) {
        System.out.println("Good evening!");
        
        int number = 5;
        System.out.println("Recursive factorial of " + number + " is: " + recursive_factorial(number));
        System.out.println("Iterative factorial of " + number + " is: " + iterative_factorial(number));
    }
    
    public static int recursive_factorial(int n) {
        if (n == 0 || n == 1) {
            return 1;
        } else {
            return n * recursive_factorial(n - 1);
        }
    }

    public static int iterative_factorial(int n) {
        int result = 1;
        for (int i = 2; i <= n; i++) {
            result *= i;
        }
        return result;
    }
}