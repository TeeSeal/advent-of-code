public class Cart {
    TrackSystem trackSystem;
    Point position;
    Orientation orientation;
    Direction turnDirection = Direction.LEFT;

    public Cart(TrackSystem ts, Point pos, Orientation orient) {
        trackSystem = ts;
        position = pos;
        orientation = orient;
    }

    public boolean hasCrashed() {
        for (Cart cart : trackSystem.carts) {
            if (cart == this) continue;
            if (position.equals(cart.position)) return true;
        }
        return false;
    }

    public Point nextPosition() {
        switch (orientation) {
            case UP: return position.up();
            case DOWN: return position.down();
            case LEFT: return position.left();
            case RIGHT: return position.right();
        }

        throw new java.lang.Error("invalid position");
    }

    public Orientation nextOrientation(char ch) {
        if (ch == '-' || ch == '|') return orientation;

        if (ch == '\\') {
            switch (orientation) {
                case UP: return Orientation.LEFT;
                case DOWN: return Orientation.RIGHT;
                case LEFT: return Orientation.UP;
                case RIGHT: return Orientation.DOWN;
            }
        }

        if (ch == '/') {
            switch (orientation) {
                case UP: return Orientation.RIGHT;
                case DOWN: return Orientation.LEFT;
                case LEFT: return Orientation.DOWN;
                case RIGHT: return Orientation.UP;
            }
        }

        if (ch == '+') {
            return orientation.turn(turnDirection);
        }

        throw new java.lang.Error("invalid orientation for " + ch);
    }

    public Direction nextTurn() {
        switch (turnDirection) {
            case LEFT: return Direction.AHEAD;
            case AHEAD: return Direction.RIGHT;
            case RIGHT: return Direction.LEFT;
        }

        throw new java.lang.Error("invalid turn");
    }

    public void move() {
        position = nextPosition();
        char ch = trackSystem.charAt(position);
        orientation = nextOrientation(ch);

        if (ch == '+') {
            turnDirection = nextTurn();
        }
    }

    public int compare(Cart other) {
        return position.compare(other.position);
    }
}
