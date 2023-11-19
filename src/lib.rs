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
        let _cases = HashMap::from([("* * * * *", "At every minute")]);
    }

    #[test]
    fn test_categorise() {
        let expr = "1-2,1-3/2,1/2,2";
        let expected = vec![
            Minutes::new(Some("1".to_string()), Some("2".to_string()), None, None),
            Minutes::new(
                Some("1".to_string()),
                Some("3".to_string()),
                Some("2".to_string()),
                None,
            ),
            Minutes::new(
                Some("1".to_string()),
                Some("59".to_string()),
                Some("2".to_string()),
                None,
            ),
            Minutes::new(None, None, None, Some("2".to_string())),
        ];
        let seg = Minutes::default();
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

        test_cases.into_iter().for_each(|(case, expected)| {
            let min = Minutes::default();
            assert_eq!(categorize(case, &min).is_ok(), expected);
        });
    }

    #[test]
    #[should_panic]
    fn test_invalid_range() {
        let expr = "70 4 10 JAN 3";
        let _r = CronEntry::from_str(expr).unwrap();
    }

    #[test]
    fn test_str_representation() {
        let expr = "10,3-10 * * * *";
        let r = CronEntry::from_str(expr).unwrap();
        println!("Cron entry {}", r);
        assert_eq!(true, false)
    }
}
