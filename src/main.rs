use crepe::crepe;

crepe! {
    @input
    struct Ship(u8, u8);

    @input
    struct Shot(u8, u8);

    @output
    #[derive(Debug)]
    struct Miss(u8, u8);

    @output
    #[derive(Debug)]
    struct Undamaged(u8, u8);

    @output
    #[derive(Debug)]
    struct Hit(u8, u8);

    @output
    #[derive(Debug)]
    struct Sink(u8, u8);

    struct Related(u8, u8, u8, u8);
    struct RelatedUndamaged(u8, u8);

    Hit(x, y) <- Ship(x, y), Shot(x, y);
    Miss(x, y) <- Shot(x, y), !Ship(x, y);
    Undamaged(x, y) <- Ship(x, y), !Shot(x, y);

    Related(x, y, x + 1, y) <- Ship(x, y), Ship(x + 1, y);
    Related(x, y, x, y + 1) <- Ship(x, y), Ship(x, y + 1);
    Related(x1, y1, x2, y2) <- Related(x2, y2, x1, y1);

    Related(x1, y1, x2, y2) <-
        Related(x1, y1, a, b), Related(a, b, x2, y2);

    RelatedUndamaged(x, y) <-
        Related(x, y, u, v), Undamaged(u, v);

    Sink(x, y) <- Hit(x, y), !RelatedUndamaged(x, y);
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn ships(s: &str) -> HashSet<Ship> {
        s.chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '#' => Some(Ship(i as u8, 0)),
                '.' | _ => None,
            })
            .collect()
    }

    fn shots(s: &str) -> HashSet<Shot> {
        s.chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '*' => Some(Shot(i as u8, 0)),
                '.' | _ => None,
            })
            .collect()
    }

    fn misses(s: &str) -> HashSet<Miss> {
        s.chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '.' => Some(Miss(i as u8, 0)),
                _ => None,
            })
            .collect()
    }

    fn undamaged(s: &str) -> HashSet<Undamaged> {
        s.chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '#' => Some(Undamaged(i as u8, 0)),
                _ => None,
            })
            .collect()
    }

    fn hits(s: &str) -> HashSet<Hit> {
        s.chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '*' | 'X' => Some(Hit(i as u8, 0)),
                _ => None,
            })
            .collect()
    }

    fn sinks(s: &str) -> HashSet<Sink> {
        s.chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                'X' => Some(Sink(i as u8, 0)),
                _ => None,
            })
            .collect()
    }

    fn expec(s: &str) -> (HashSet<Miss>, HashSet<Undamaged>, HashSet<Hit>, HashSet<Sink>) {
        (misses(s), undamaged(s), hits(s), sinks(s))
    }

    #[test]
    fn all_misses() {
        let mut runtime = Crepe::new();
        runtime.extend(ships(".#.."));
        runtime.extend(shots("*.**"));
        assert_eq!(runtime.run(), expec(".#.."));
    }

    #[test]
    fn one_hit_two_undamaged() {
        let mut runtime = Crepe::new();
        runtime.extend(ships("###"));
        runtime.extend(shots(".*."));
        assert_eq!(runtime.run(), expec("#*#"));
    }

    #[test]
    fn three_sink() {
        let mut runtime = Crepe::new();
        runtime.extend(ships("###"));
        runtime.extend(shots("***"));
        assert_eq!(runtime.run(), expec("XXX"));
    }
}
