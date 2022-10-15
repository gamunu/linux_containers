#[cfg(test)]
mod tests {
    use super::super::adaptor::Adaptor;

    use std::collections::HashMap;

    #[test]
    fn test_filters() {
        #[derive(Clone)]
        struct CEntry {
            name: String,
            other: String,
            labels: Option<HashMap<String, String>>,
        }

        let corpus = vec![
            CEntry {
                name: "food".to_owned(),
                other: "".to_owned(),
                labels: Some(HashMap::from([("foo".to_owned(), "true".to_owned())])),
            },
            CEntry { name: "bar".to_owned(), other: "".to_owned(), labels: None },
            CEntry {
                name: "bar".to_owned(),
                other: "".to_owned(),
                labels: Some(HashMap::from([("bar".to_owned(), "true".to_owned())])),
            },
            CEntry {
                name: "fooer".to_owned(),
                other: "".to_owned(),
                labels: Some(HashMap::from([(
                    "more complex label with \\ and \"".to_owned(),
                    "present".to_owned(),
                )])),
            },
            CEntry {
                name: "fooer".to_owned(),
                other: "".to_owned(),
                labels: Some(HashMap::from([(
                    "more complex label with \\ and \".post".to_owned(),
                    "present".to_owned(),
                )])),
            },
            CEntry { name: "baz".to_owned(), other: "too complex, yo".to_owned(), labels: None },
            CEntry { name: "bazo".to_owned(), other: "abc".to_owned(), labels: None },
            CEntry {
                name: "compound".to_owned(),
                other: "".to_owned(),
                labels: Some(HashMap::from([("foo".to_owned(), "omg_asdf.asdf-qwer".to_owned())])),
            },
        ];

        let adapt = |obj: CEntry| -> Box<dyn Adaptor> {
            Box::new(super::super::adaptor::AdapterFunc(
                move |field_path: Vec<String>| -> (String, bool) {
                    match field_path[0].as_str() {
                        "name" => return (obj.name.clone(), obj.name.chars().count() > 0),
                        "other" => return (obj.other.clone(), obj.other.chars().count() > 0),
                        "labels" => match &obj.labels {
                            Some(l) => {
                                let key = field_path[1..].join(".");
                                return (l.get(&key).unwrap().to_owned(), true);
                            }
                            None => return ("".to_owned(), false),
                        },
                        _ => return ("".to_owned(), false),
                    }
                },
            ))
        };

        #[derive(Clone)]
        struct tEntry<'a> {
            name: String,
            input: String,
            expected: Option<&'a Vec<CEntry>>,
            err_string: Option<String>,
        }

        let corpus = vec![
            tEntry {
                name: "Empty".to_owned(),
                input: "".to_owned(),
                expected: Some(&corpus),
                err_string: None,
            },
            tEntry {
                name: "Present".to_owned(),
                input: "name".to_owned(),
                expected: Some(&corpus),
                err_string: None,
            },
            tEntry {
                name: "LabelPresent".to_owned(),
                input: "labels.foo".to_owned(),
                expected: Some(&vec![corpus[0].clone(), corpus[2].clone(), corpus[8].clone()]),
                err_string: None,
            },
            tEntry {
                name: "NameAndLabelPresent".to_owned(),
                input: "labels.foo,name".to_owned(),
                expected: Some(&vec![corpus[0].clone(), corpus[2].clone(), corpus[8].clone()]),
                err_string: None,
            },
            tEntry {
                name: "LabelValue".to_owned(),
                input: "labels.foo==true".to_owned(),
                expected: Some(&vec![corpus[0].clone()]),
                err_string: None,
            },
            tEntry {
                name:  "LabelValuePunctuated".to_owned(),
                input: "labels.foo==omg_asdf.asdf-qwer".to_owned(),
                expected: Some(&vec![
                    corpus[8].clone(),
                ]),
                err_string: None,
            },
            tEntry {
                name:      "LabelValueNoAltQuoting".to_owned(),
                input:     "labels.|foo|==omg_asdf.asdf-qwer".to_owned(),
                expected: None,
                err_string: Some("filters: parse error: [labels. >|||< foo|==omg_asdf.asdf-qwer]: invalid quote encountered".to_owned()),
            },
            tEntry {
                name:  "Name".to_owned(),
                input: "name==bar".to_owned(),
                expected: Some(&vec![
                    corpus[1].clone(),
                    corpus[3].clone(),
                ]),
                err_string: None
            },
        ];
    }
}
