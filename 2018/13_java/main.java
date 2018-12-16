import java.io.*;
import java.util.*;

public class main {
    public static void main(String[] args) throws IOException {
        File file = new File("input.txt");
        BufferedReader br = new BufferedReader(new FileReader(file));

        ArrayList<String> grid = new ArrayList<String>();
        String st;
        while ((st = br.readLine()) != null) {
            grid.add(st);
        }

        br.close();

        TrackSystem ts = new TrackSystem(grid);
        boolean crashed = false;

        while (ts.carts.size() > 1) {
            ts.sortCarts();

            for (Cart cart : ts.carts) {
                if (cart.hasCrashed()) continue;
                cart.move();

                if (cart.hasCrashed() && !crashed) {
                    crashed = true;
                    System.out.println("Part 1: " + cart.position);
                }
            }

            ts.carts.removeIf((c) -> c.hasCrashed());
        }

        System.out.println("Part 2: " + ts.carts.get(0).position);
    }
}
