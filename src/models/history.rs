use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct History {
    pub id: i32,
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: String,
    pub created_at: String
}

impl History {
    fn extract_date(&self) -> String {
        self.created_at.split_whitespace().next().unwrap().to_string()
    }

    pub fn group_histories(histories: Vec<History>, inital: Option<HashMap<String, Vec<History>>>)->HashMap<String, Vec<History>> {
        let mut grouped_histories: HashMap<String, Vec<History>> = match inital {
            Some(value) => {
                value
            },
            None=>{
                HashMap::new()
            }
        };

        for history in histories {
            let date = history.extract_date();
            grouped_histories
                .entry(date)
                .or_insert_with(Vec::new)
                .push(history);
        }
        grouped_histories
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_histories() {
        let history1 = History {
            id: 1,
            method: "GET".to_string(),
            url: "http://example.com/resource1".to_string(),
            body: "".to_string(),
            headers: "Content-Type: application/json".to_string(),
            created_at: "2024-04-09 12:00:00".to_string(),
        };
        let history2 = History {
            id: 2,
            method: "POST".to_string(),
            url: "http://example.com/resource2".to_string(),
            body: "".to_string(),
            headers: "Content-Type: application/xml".to_string(),
            created_at: "2024-04-09 13:00:00".to_string(),
        };
        let history3 = History {
            id: 3,
            method: "PUT".to_string(),
            url: "http://example.com/resource3".to_string(),
            body: "".to_string(),
            headers: "Content-Type: text/plain".to_string(),
            created_at: "2024-04-10 12:00:00".to_string(),
        };

        let histories = vec![history1, history2, history3];
        let grouped = History::group_histories(histories, Option::None);

        assert_eq!(grouped.len(), 2); // We expect two groups, one for each date

        assert!(grouped.contains_key("2024-04-09"));
        assert!(grouped.contains_key("2024-04-10"));

        let group1 = grouped.get("2024-04-09").unwrap();
        assert_eq!(group1.len(), 2); // Two histories on 2024-04-09

        let group2 = grouped.get("2024-04-10").unwrap();
        assert_eq!(group2.len(), 1); // One history on 2024-04-10
    }
}