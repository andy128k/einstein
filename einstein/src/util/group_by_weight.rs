use std::iter::Iterator;

pub fn group_by_weight<T, I, F>(iter: I, fits_to_bag: F) -> Vec<Vec<T>>
where
    I: Iterator<Item = T>,
    F: Fn(&[T], &T) -> bool,
{
    let mut bags = Vec::new();
    let mut bag = Vec::new();
    for item in iter {
        if bag.is_empty() || fits_to_bag(&bag, &item) {
            bag.push(item);
        } else {
            bags.push(bag);
            bag = vec![item];
        }
    }
    if !bag.is_empty() {
        bags.push(bag);
    }
    bags
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::join;

    #[test]
    fn test_group() {
        let v = vec![1, 2, 3];
        assert_eq!(
            group_by_weight(v.iter(), |_, _| false),
            vec![vec![&1], vec![&2], vec![&3]]
        );
    }

    #[test]
    fn test_group2() {
        let v = vec![
            "Lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet,",
            "consectetur",
            "adipiscing",
            "elit,",
            "sed",
            "do",
            "eiusmod",
            "tempor",
            "incididunt",
            "ut",
            "labore",
            "et",
            "dolore",
            "magna",
            "aliqua.",
            "Ut",
            "enim",
            "ad",
            "minim",
            "veniam,",
            "quis",
            "nostrud",
            "exercitation",
            "ullamco",
            "laboris",
            "nisi",
            "ut",
            "aliquip",
            "ex",
            "ea",
            "commodo",
            "consequat.",
            "Duis",
            "aute",
            "irure",
            "dolor",
            "in",
            "reprehenderit",
            "in",
            "voluptate",
            "velit",
            "esse",
            "cillum",
            "dolore",
            "eu",
            "fugiat",
            "nulla",
            "pariatur.",
            "Excepteur",
            "sint",
            "occaecat",
            "cupidatat",
            "non",
            "proident,",
            "sunt",
            "in",
            "culpa",
            "qui",
            "officia",
            "deserunt",
            "mollit",
            "anim",
            "id",
            "est",
            "laborum.",
        ];
        assert_eq!(
            group_by_weight(v.iter(), |b, i| (join(b, " ") + " " + i).len() <= 25),
            vec![
                vec![&"Lorem", &"ipsum", &"dolor", &"sit"],
                vec![&"amet,", &"consectetur"],
                vec![&"adipiscing", &"elit,", &"sed", &"do"],
                vec![&"eiusmod", &"tempor", &"incididunt"],
                vec![&"ut", &"labore", &"et", &"dolore", &"magna"],
                vec![&"aliqua.", &"Ut", &"enim", &"ad", &"minim"],
                vec![&"veniam,", &"quis", &"nostrud"],
                vec![&"exercitation", &"ullamco"],
                vec![&"laboris", &"nisi", &"ut", &"aliquip"],
                vec![&"ex", &"ea", &"commodo", &"consequat."],
                vec![&"Duis", &"aute", &"irure", &"dolor", &"in"],
                vec![&"reprehenderit", &"in"],
                vec![&"voluptate", &"velit", &"esse"],
                vec![&"cillum", &"dolore", &"eu", &"fugiat"],
                vec![&"nulla", &"pariatur.", &"Excepteur"],
                vec![&"sint", &"occaecat", &"cupidatat"],
                vec![&"non", &"proident,", &"sunt", &"in"],
                vec![&"culpa", &"qui", &"officia"],
                vec![&"deserunt", &"mollit", &"anim", &"id"],
                vec![&"est", &"laborum."],
            ]
        );
    }
}
