use crepe::crepe;

crepe! {
    @input
    struct Ship(i32, i32);

    @input
    struct Shot(i32, i32);

    @output
    struct Miss(i32, i32);

    @output
    struct Undamaged(i32, i32);

    @output
    struct Hit(i32, i32);

    @output
    struct Sink(i32, i32);

    struct Related(i32, i32, i32, i32);
    struct RelatedUndamaged(i32, i32);

    Hit(x, y) <- Ship(x, y), Shot(x, y);
    Miss(x, y) <- Shot(x, y), !Ship(x, y);
    Undamaged(x, y) <- Ship(x, y), !Shot(x, y);

    Related(x, y, x + 1, y) <- Ship(x, y), Ship(x + 1, y);
    Related(x, y, x - 1, y) <- Ship(x, y), Ship(x - 1, y);
    Related(x, y, x, y + 1) <- Ship(x, y), Ship(x, y + 1);
    Related(x, y, x, y - 1) <- Ship(x, y), Ship(x, y - 1);

    Related(x1, y1, x2, y2) <-
        Related(x1, y1, a, b), Related(a, b, x2, y2);

    RelatedUndamaged(x, y) <-
        Related(x, y, u, v), Undamaged(u, v);

    Sink(x, y) <- Hit(x, y), !RelatedUndamaged(x, y);
}

fn main() {
    println!("Hello, world!");
}
