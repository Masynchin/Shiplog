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

    #[test]
    fn all_misses() {
        let mut runtime = Crepe::new();
        runtime.extend(ships(".#.."));
        runtime.extend(shots("*.**"));

        let (misses, _, _, _) = runtime.run();

        assert_eq!(misses, HashSet::from([Miss(0, 0), Miss(2, 0), Miss(3, 0)]));
    }

    #[test]
    fn one_hit_two_undamaged() {
        let mut runtime = Crepe::new();
        runtime.extend(ships("###"));
        runtime.extend(shots(".*."));

        let (_, undamaged, hits, _) = runtime.run();

        assert_eq!(hits, HashSet::from([Hit(1, 0)]));
        assert_eq!(undamaged, HashSet::from([Undamaged(0, 0), Undamaged(2, 0)]));
    }

    #[test]
    fn three_sink() {
        let mut runtime = Crepe::new();
        runtime.extend(ships("###"));
        runtime.extend(shots("***"));

        let (_, _, _, sinked) = runtime.run();

        assert_eq!(sinked, HashSet::from([Sink(0, 0), Sink(1, 0), Sink(2, 0)]));
    }
}
