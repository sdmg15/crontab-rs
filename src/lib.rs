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
        let _cases = HashMap::from(
            [
                ("23 0-20/2 * * *", "At minute 23 Past every 2 hour from 0 through 20"),
                ("0 0,12 1 */2 *", "At minute 0 past hour 0 and 12 on day-of-month 1 in every 2nd month"),
                ("* * * * *", "At every minute"),
                ("* 1,2-3/1 * * *", "At every minute past hour 1 and Past every 1 hour from 2 through 3"),
                ("* * 1-3/2,2,1 * *", "At every minute  on every 2 day-of-month from 1 through 3 and On day-of-month 2 and On day-of-month 1"),
                ("* * 2,3 1-2/3,2,3 *", "At every minute  On day-of-month 2 and On day-of-month 3 in every 3 month from 1 through 2 and in month 2 and in month 3"),
                ("* * 2,3 1-2/3,2,3 1-2/2", "“At every minute on day-of-month 2 and 3 and on every 2nd day-of-week from Monday through Tuesday in every 3rd month from January through February, February, and March.")
            ]
        );
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
        let expr = "* 1,2-3/1 * * *";
        let r = CronEntry::from_str(expr).unwrap();
        println!("Cron entry {}", r);
        assert_eq!(true, false)
    }
}
