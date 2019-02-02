import java.util.*;

public class TrackSystem {
    public ArrayList<String> grid;
    public ArrayList<Cart> carts;

    public TrackSystem(ArrayList<String> grid) {
        this.grid = grid;
        carts = new ArrayList<Cart>();

        for (int y = 0; y < grid.size(); y++) {
            String line = grid.get(y);
            for (int x = 0; x < line.length(); x++) {
                char ch = line.charAt(x);
                if (ch == '^' || ch == 'v' || ch == '<' || ch == '>') {
                    Cart cart = new Cart(this, new Point(x, y), Orientation.forChar(ch));
                    carts.add(cart);
                }
            }
        }
    }

    public char charAt(int x, int y) {
        if (y >= grid.size()) return ' ';
        String line = grid.get(y);
        if (x >= line.length()) return ' ';
        char ch = line.charAt(x);

        if (ch == '>' || ch == '<') return '-';
        if (ch == 'v' || ch == '^') return '|';

        return ch;
    }

    public char charAt(Point point) {
        return charAt(point.x, point.y);
    }

    public void sortCarts() {
        carts.sort((a, b) -> a.compare(b));
    }

    public void removeCrashedCarts() {
        carts.removeIf((c) -> c.hasCrashed());
    }
}
