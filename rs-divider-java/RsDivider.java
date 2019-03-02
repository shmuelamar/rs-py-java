class RsDivider {
    private static native double divNumbers(double a, double b);

    static {
        System.loadLibrary("rsdividerjava");
    }

    public static void main(String[] args) {
        double result = RsDivider.divNumbers(4, 2);
        assert result == 2;
        System.out.println("4 / 2 = " + result);
    }
}
