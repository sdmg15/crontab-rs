mod logic;
mod types;

#[cfg(test)]

mod tests {

    use super::*;
    use logic::*;
    use std::collections::HashMap;
    use std::str::FromStr;
    use types::*;

    #[test]

    fn test_to_string() {
        let cases = HashMap::from([("* * * * *", "At every minute")]);
    }

    #[test]
    fn test_categorise() {
        let expr = "1-2,1-3/2,1/2,2";
        let expected = vec![
            Minutes {
                range_start: Some("1".to_string()),
                range_end: Some("2".to_string()),
                step: None,
                val: None,
            },
            Minutes {
                range_start: Some("1".to_string()),
                range_end: Some("3".to_string()),
                step: Some("2".to_string()),
                val: None,
            },
            Minutes {
                range_start: Some("1".to_string()),
                range_end: Some("59".to_string()),
                step: Some("2".to_string()),
                val: None,
            },
            Minutes {
                range_start: None,
                range_end: None,
                step: None,
                val: Some("2".to_string()),
            },
        ];
        let seg = Minutes::new();
        let r = categorize(expr, &seg).unwrap();
        assert_eq!(expected, r);
    }

    #[test]
    fn test_parse_range() {
        let test_cases = HashMap::from([
            ("1-12", true),
            ("*-*", false),
            ("*", true),
            ("12-1", false),
            ("12-12", true),
            ("*-10", false),
            ("1-10/2", true),
            ("10,11,0-12", true),
            ("10,11,12-9", false),
            ("10,11,9-12", true),
            ("19,11,9-12/3", true),
            ("19,*,9-12/3", true),
        ]);

        //        test_cases.into_iter().for_each(|(case, expected)| {
        //            let min = Minutes(case.to_string());
        //            assert_eq!(validate(&min), expected);
        //        });
    }

    #[test]
    #[should_panic]
    fn test_invalid_range() {
        let expr = "70 4 10 JAN 3";
        let r = CronEntry::from_str(expr).unwrap();
    }
}
