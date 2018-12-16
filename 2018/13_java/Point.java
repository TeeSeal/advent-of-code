public class Point {
    public int x;
    public int y;

    public Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int compare(Point other) {
        if (y > other.y) return 1;
        if (y < other.y) return -1;

        if (x > other.x) return 1;
        if (x < other.x) return -1;

        return 0;
    }

    public Point up() {
        return new Point(x, y - 1);
    }

    public Point down() {
        return new Point(x, y + 1);
    }

    public Point left() {
        return new Point(x - 1, y);
    }

    public Point right() {
        return new Point(x + 1, y);
    }

    public String toString() {
        return "{" + x + ", " + y + "}";
    }

    public boolean equals(Point other) {
        return x == other.x && y == other.y;
    }
}
