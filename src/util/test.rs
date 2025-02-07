#[cfg(test)]
mod dates {
    use crate::Date;

    #[test]
    fn date_gen_simple() {
        let vec_date_data = vec!["2020-01-01", "2020-01-02", "2020-01-03", "2020-01-04", "2020-01-05", "2020-01-06", "2020-01-07", "2020-01-08", "2020-01-09", "2020-01-10"];
        let vec_date = vec_date_data.iter().map(|x| Date::from(x)).collect::<Vec<Date>>();
        assert_eq!(vec_date[0], Date::new(2020, 1, 1));
        assert_eq!(vec_date[1], Date::new(2020, 1, 2));
        assert_eq!(vec_date[2], Date::new(2020, 1, 3));
        assert_eq!(vec_date[3], Date::new(2020, 1, 4));
        assert_eq!(vec_date[4], Date::new(2020, 1, 5));
        assert_eq!(vec_date[5], Date::new(2020, 1, 6));
        assert_eq!(vec_date[6], Date::new(2020, 1, 7));
        assert_eq!(vec_date[7], Date::new(2020, 1, 8));
        assert_eq!(vec_date[8], Date::new(2020, 1, 9));
        assert_eq!(vec_date[9], Date::new(2020, 1, 10));
    }

    #[test]
    fn date_gen_faulty() {
        let vec_date_data = vec!["", "xxXx-yy-zz", "2014.15.10", "asdf-xcv", "25-10", "15-10-10", "2010-DEC-11", "2010-15-35"];
        let vec_date = vec_date_data.iter().map(|x| Date::from(x)).collect::<Vec<Date>>();

        assert_eq!(vec_date[0], Date::default());
        assert_eq!(vec_date[1], Date::default());
        assert_eq!(vec_date[2], Date::default());
        assert_eq!(vec_date[3], Date::default());
        assert_eq!(vec_date[4], Date::default());
        assert_eq!(vec_date[5], Date::default());
        assert_eq!(vec_date[6], Date::default());
        assert_eq!(vec_date[7], Date::default());
    }

    #[test]
    fn date_gen_loop_10k() {
        for n in 1..10_000 {
            let month = {
                let tmp = n % 12;
                if tmp == 0 {
                    1
                } else {
                    tmp as u8
                }
            };
            let day = {
                let tmp = n % 31;
                if tmp == 0 {
                    1
                } else {
                    tmp as u8
                }
            };
            let date = Date::new(n, month, day);
            assert_eq!(date, Date::from(format!("{:04}-{:02}-{:02}", n, month, day)));
        }
    }
}
