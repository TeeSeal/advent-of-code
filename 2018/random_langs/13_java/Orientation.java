enum Orientation {
    UP, DOWN, LEFT, RIGHT;

    public static Orientation forChar(char ch) {
        switch (ch) {
            case '^': return UP;
            case 'v': return DOWN;
            case '>': return RIGHT;
            case '<': return LEFT;
        }
        return UP;
    }

    public Orientation left() {
        switch (this) {
            case UP: return LEFT;
            case DOWN: return RIGHT;
            case LEFT: return DOWN;
            case RIGHT: return UP;
        }
        return UP;
    }

    public Orientation right() {
        switch (this) {
            case UP: return RIGHT;
            case DOWN: return LEFT;
            case LEFT: return UP;
            case RIGHT: return DOWN;
        }
        return UP;
    }

    public Orientation turn(Direction dir) {
        switch (dir) {
            case AHEAD: return this;
            case LEFT: return left();
            case RIGHT: return right();
        }

        return this;
    }
}
