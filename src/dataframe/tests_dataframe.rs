
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

    #[test]
    fn test_read_by() {
        let mut df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let series = df.by("Profit").unwrap();
        assert_eq!(series.label, "Profit".to_string());
    }

    #[test]
    fn test_read_many() {
        let mut df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let series = df.many(vec!["Profit", "State"]);
        assert_eq!(series.len(), 2);
    }

    #[test]
    fn test_index() {
        let mut df = DataFrame::new(
            vec![
                    row![0.4, 0.7, "book", true, 1],
                    row![3.0, 4.7, "poster", true, 1],
                 ],
            vec!["A", "B", "C", "D", "E"]
        );
        let series = df["A"].clone();
        assert_eq!(series.label, "A");
        let series = df[0].clone();
        assert_eq!(series.label, "A");
    }

    #[test]
    fn test_map() {
        let mut df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let mut new_keys = HashMap::new();
        new_keys.insert("New York", 0 as u32);
        let mut series = df.map("State", new_keys);
        let mut data = series.by("State").unwrap().clone().contains("0");
        assert_eq!(data, true);
    }

    #[test]
    fn test_convert_from_vector() {
        let mut df = DataFrame::from_vec(vec![vec![21,23]], vec!["A", "B"]);
        assert_eq!(df.size, 1);
    }

    #[test]
    fn test_dummies_test() {
        let mut df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let new_df = df.get_dummies("State");
        assert_eq!(df.size, 50);
    }

    #[test]
    fn test_concate() {