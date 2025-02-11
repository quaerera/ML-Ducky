
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
        let mut df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let new_df = df.get_dummies("State");
        let best_df = df.concat(new_df);
        assert_eq!(best_df.size, 8);
    }

    #[test]
    fn test_drop() {
        let mut df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let new_df = df.drop(vec!["State"]).unwrap();
        assert_eq!(new_df.contains("State"), false);
    }

    #[test]
    fn test_join() {
        let mut df = DataFrame::new(
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]
        );
        let mut df2 = DataFrame::new(
            vec![
                row![1.4, 1.7],
                row![13.0, 13.7],
            ],
            vec!["F", "G"]
        );
        df.join(df2);
        assert_eq!(df.contains("G"), true);
    }

    #[test]
    fn test_to_rows() {
        let mut df = DataFrame::new(
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]
        );
        let result = df.to_rows().unwrap();
        assert_eq!(result[0].len(), 5);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_group_by() {
        let mut df = DataFrame::new(
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
                row![3.0, 4.7, "book", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]
        );
        let result = df.group_by("C").unwrap();
        assert_eq!(result.contains_key("book"), true);
    }

    #[test]
    fn test_sort() {
        let mut df = DataFrame::new(
            vec![
                row![0.4, 0.7, "poster", true, 1],
                row![3.0, 4.7, "table", true, 1],
                row![3.0, 4.7, "book", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]
        );
        let result = df.sort("C").unwrap();
        assert_eq!(result.data[2].data[0], Cell::Text("book".to_string()));
    }

    #[test]
    fn test_to_matrix() {
        let mut df = DataFrame::new(
            vec![
                row![0.4, 0.7, true, 1],
                row![3.0, 4.7, true, 1],
                row![3.0, 4.7, true, 1],
            ],
            vec!["A", "B", "D", "E"]
        );
        let result = df.to_matrix().unwrap();
        assert_eq!(result[0][2], 1.0);
    }
}