# Duck ML\n### Duck is implementation in Rust Machine Learning Library.\nIn this moment we start creating DataFrame.\nWe want to use ndarray as some numpy alternative and nalgebra as mathematic part of coding.\n\n#### Contributors:\n    quaerera\n#### Ducky's Structures:\n    type DataFrame -> Data Enginnering\n        eg: \n        DataFrame::new(\n                    vec![\n                        row![0.4, 0.7, "poster", true, 1],\n                        row![3.0, 4.7, "table", true, 1],\n                        row![3.0, 4.7, "book", true, 1],\n                    ],\n                    vec!["A", "B", "C", "D", "E"]\n                );\n    type DataFrameGroupBy -> Hashmap\n    type Series -> Vectorize structure\n    and many more.\n\n#### Ducky's Operations:\n    fn push(&mut self, element: Vec<Cell>);\n    and many more.\n\n#### Ducky's Science:\n    fn get_dummies(&mut self, label: &str) -> DataFrame;\n    and many more.\n\n#### Ducky's Math:\n    fn sig