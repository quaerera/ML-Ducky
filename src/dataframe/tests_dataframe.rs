
#[cfg(test)]

mod tests {
    use std::collections::HashMap;

    use crate::dataframe::DataFrame;
    use crate::dataframe::operations::Operations;
    use crate::dataframe::science::Science;
    use crate::series::SeriesImpl;
    use crate::cell::Cell;

    #[test]
    fn test_create_dataframe_from_vec() {
        let df = DataFrame::new(
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]

        );
        assert_eq!(df.size, 2);
    }

    #[test]
    fn test_push_dataframe_from_vec() {
        let mut df = DataFrame::new(
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]
        );
        df.push(row![0.4, 0.7, "book", true, 1]);
        assert_eq!(df.size, 3);
    }

    #[test]
    fn test_create_from_csv_dataframe() {
        let df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        assert_eq!(df.size, 50);
    }