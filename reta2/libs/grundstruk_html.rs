// grundstruk_html.rs
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use serde_json::{Value, Map};

// Type alias for our ordered dictionary structure
type OrderedDict = BTreeMap<String, OrderedValue>;

#[derive(Debug, Clone)]
enum OrderedValue {
    Dict(OrderedDict),
    Null,
    String(String),
}

impl OrderedValue {
    fn is_dict(&self) -> bool {
        matches!(self, OrderedValue::Dict(_))
    }
    
    fn as_dict(&self) -> Option<&OrderedDict> {
        match self {
            OrderedValue::Dict(dict) => Some(dict),
            _ => None,
        }
    }
    
    fn as_dict_mut(&mut self) -> Option<&mut OrderedDict> {
        match self {
            OrderedValue::Dict(dict) => Some(dict),
            _ => None,
        }
    }
}

// Custom comparison function similar to Python's cmp_before
fn cmp_before(value: &str) -> (bool, String) {
    let is_number: bool;
    let to_sort: String;
    
    if value.contains('/') {
        let parts: Vec<&str> = value.split('/').collect();
        if let Some(last_part) = parts.last() {
            if last_part.chars().all(|c| c.is_ascii_digit()) {
                is_number = true;
                to_sort = last_part.to_string();
            } else {
                is_number = false;
                to_sort = value.to_string();
            }
        } else {
            is_number = false;
            to_sort = value.to_string();
        }
    } else if value.chars().all(|c| c.is_ascii_digit()) {
        is_number = true;
        to_sort = value.to_string();
    } else {
        is_number = false;
        to_sort = value.to_string();
    }
    
    (is_number, to_sort)
}

// Custom comparison function similar to Python's cmpx
fn cmpx(erster: &str, zweiter: &str) -> Ordering {
    let (is_number1, value1) = cmp_before(erster);
    let (is_number2, value2) = cmp_before(zweiter);
    
    match (is_number1, is_number2) {
        (true, true) => {
            if let (Ok(num1), Ok(num2)) = (value1.parse::<i64>(), value2.parse::<i64>()) {
                match num1.cmp(&num2) {
                    Ordering::Equal => {
                        // Special handling for values with "/"
                        match (erster.contains('/'), zweiter.contains('/')) {
                            (true, false) => Ordering::Greater,
                            (false, true) => Ordering::Less,
                            _ => Ordering::Equal,
                        }
                    }
                    other => other,
                }
            } else {
                value1.cmp(&value2)
            }
        }
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => value1.cmp(&value2),
    }
}

// Function to compare key-value pairs for sorting
fn cmp_key_value_pair(
    (key1, _): &(String, OrderedValue),
    (key2, _): &(String, OrderedValue),
) -> Ordering {
    cmpx(key1, key2)
}

// Merge dictionaries similar to Python's merge_dicts
fn merge_dicts(dict1: &mut OrderedDict, dict2: &OrderedDict) {
    for (key, value2) in dict2 {
        if let Some(value1) = dict1.get_mut(key) {
            match (value1, value2) {
                (OrderedValue::Dict(dict1_inner), OrderedValue::Dict(dict2_inner)) => {
                    merge_dicts(dict1_inner, dict2_inner);
                }
                (OrderedValue::Dict(dict1_inner), OrderedValue::String(str2)) => {
                    // Special case: convert dict1_inner to a dict containing the string
                    let mut new_dict = OrderedDict::new();
                    for (k, v) in dict1_inner.iter() {
                        new_dict.insert(k.clone(), v.clone());
                    }
                    new_dict.insert(str2.clone(), OrderedValue::Null);
                    *value1 = OrderedValue::Dict(new_dict);
                }
                _ => {
                    *value1 = value2.clone();
                }
            }
        } else {
            dict1.insert(key.clone(), value2.clone());
        }
    }
    
    // Sort the dictionary
    let mut sorted_items: Vec<(String, OrderedValue)> = dict1.drain().collect();
    sorted_items.sort_by(cmp_key_value_pair);
    for (k, v) in sorted_items {
        dict1.insert(k, v);
    }
}

// Traverse hierarchy similar to Python's traverseHierarchy
fn traverse_hierarchy(
    liste: &[String],
    thing: OrderedDict,
    listen_index: usize,
    value: &str,
) -> OrderedDict {
    let mut knoten = liste[listen_index].clone();
    knoten = knoten.replace("pro", "/");
    
    let mut thing_dict = OrderedDict::new();
    thing_dict.insert(knoten.clone(), OrderedValue::Dict(thing));
    
    if listen_index == 0 {
        let new_keys: Vec<String> = value.split(',').map(|s| s.trim().to_string()).collect();
        let mut value_dict = OrderedDict::new();
        for key in new_keys {
            value_dict.insert(key, OrderedValue::Null);
        }
        
        // Sort the value dictionary
        let mut sorted_items: Vec<(String, OrderedValue)> = value_dict.drain().collect();
        sorted_items.sort_by(cmp_key_value_pair);
        for (k, v) in sorted_items {
            value_dict.insert(k, v);
        }
        
        thing_dict.insert(knoten, OrderedValue::Dict(value_dict));
    }
    
    if listen_index + 1 < liste.len() {
        if let Some(OrderedValue::Dict(mut inner_dict)) = thing_dict.remove(&liste[listen_index]) {
            inner_dict = traverse_hierarchy(liste, inner_dict, listen_index + 1, value);
            thing_dict.insert(liste[listen_index].clone(), OrderedValue::Dict(inner_dict));
        }
    }
    
    thing_dict
}

// Main function to process the data structure
fn process_wahl_structure(wahl15: &OrderedDict) -> OrderedDict {
    let mut wahl_neu = OrderedDict::new();
    
    for (key, value) in wahl15 {
        let mut key_with_underscore = format!("_{}", key);
        let liste: Vec<String> = key_with_underscore
            .split('_')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        if !liste.is_empty() {
            let mut reversed_liste: Vec<String> = liste.iter().rev().cloned().collect();
            let thing = traverse_hierarchy(&reversed_liste, OrderedDict::new(), 0, value);
            merge_dicts(&mut wahl_neu, &thing);
        }
    }
    
    let mut wahl_neu2 = OrderedDict::new();
    wahl_neu2.insert("15".to_string(), OrderedValue::Dict(wahl_neu.clone()));
    
    // Merge with the "15" subtree
    if let Some(OrderedValue::Dict(mut fifteen_dict)) = wahl_neu2.get("15").cloned() {
        merge_dicts(&mut fifteen_dict, &wahl_neu);
        wahl_neu2.insert("15".to_string(), OrderedValue::Dict(fifteen_dict));
    }
    
    wahl_neu2
}

// Function to print the HTML structure
fn myprint(d: &OrderedDict, tiefe: usize, blank: bool) -> String {
    let mut output = String::new();
    let items: Vec<(&String, &OrderedValue)> = if tiefe < 2 {
        d.iter().collect()
    } else {
        d.iter().rev().collect()
    };
    
    for (k, v) in items {
        let bereich_len = match v {
            OrderedValue::Dict(v_dict) => v_dict.len() > 1 || tiefe < 2,
            _ => tiefe < 2,
        };
        
        let listen_vergleich = match v {
            OrderedValue::Dict(v_dict) => {
                (v_dict.values().any(|v_val| match v_val {
                    OrderedValue::Null => false,
                    OrderedValue::String(_) => true,
                    OrderedValue::Dict(_) => true,
                }) && v_dict.len() > 1) || tiefe < 2
            }
            _ => tiefe < 2,
        };
        
        if bereich_len {
            output.push_str(&format!(
                "<div style=\"white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);\">"
            ));
        }
        
        match v {
            OrderedValue::Null => {
                if blank {
                    output.push_str(&format!(
                        "<input type=\"checkbox\" class=\"ordGru\" onchange=\"toggleP2(this,-10,'✗','{}','{}');\" id=\"ordGru{}\" value=\"{}\">",
                        "Grundstrukturen", // This should come from i18n
                        k,
                        k,
                        k
                    ));
                } else {
                    output.push_str(&format!("<input type=\"checkbox\">"));
                }
            }
            _ => {}
        }
        
        match v {
            OrderedValue::Null => {
                if listen_vergleich {
                    let kkk = format!("<label id=\"ordGruB{}\">{}</label>", 
                        k, 
                        k.replace("_", " ")
                    );
                    output.push_str(&format!("{} ", kkk));
                }
                output.push_str("</input>");
            }
            OrderedValue::Dict(v_dict) => {
                if listen_vergleich {
                    output.push_str(&format!("{} ", k));
                }
                output.push_str(&myprint(v_dict, tiefe + 1, blank));
            }
            OrderedValue::String(s) => {
                output.push_str(&format!("{} ", s));
            }
        }
        
        if bereich_len {
            output.push_str("</div>");
        }
    }
    
    output
}

// Load data from JSON file (similar to wahl15 in Python)
fn load_wahl15_data(file_path: &str) -> Result<OrderedDict, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let json_value: Value = serde_json::from_str(&content)?;
    
    fn convert_value(value: &Value) -> OrderedValue {
        match value {
            Value::Object(obj) => {
                let mut dict = OrderedDict::new();
                for (k, v) in obj {
                    dict.insert(k.clone(), convert_value(v));
                }
                OrderedValue::Dict(dict)
            }
            Value::Null => OrderedValue::Null,
            Value::String(s) => OrderedValue::String(s.clone()),
            Value::Number(n) => OrderedValue::String(n.to_string()),
            Value::Bool(b) => OrderedValue::String(b.to_string()),
            Value::Array(arr) => {
                // Convert arrays to comma-separated strings like in Python
                let strings: Vec<String> = arr.iter()
                    .map(|v| match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => "".to_string(),
                    })
                    .collect();
                OrderedValue::String(strings.join(","))
            }
        }
    }
    
    if let Value::Object(obj) = json_value {
        let mut dict = OrderedDict::new();
        for (k, v) in obj {
            dict.insert(k, convert_value(&v));
        }
        Ok(dict)
    } else {
        Ok(OrderedDict::new())
    }
}

// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let blank = args.len() > 1 && args[1] == "blank";
    
    // Load the data (you would need to provide the actual data source)
    // For now, let's create a sample structure similar to what Python code expects
    let mut wahl15_sample = OrderedDict::new();
    
    // Add some sample data
    wahl15_sample.insert(
        "relativer_Zeit-Betrag_(15_10_4_18_6)".to_string(),
        OrderedValue::String("value1,value2,value3".to_string())
    );
    
    wahl15_sample.insert(
        "another_key_1_2_3".to_string(),
        OrderedValue::String("item1,item2".to_string())
    );
    
    let wahl_structure = process_wahl_structure(&wahl15_sample);
    
    println!(
        "<div style=\"{}\"{}>",
        if blank && false { "display:none;" } else { "" },
        if blank { "id='grundstrukturenDiv'" } else { "" }
    );
    
    let html_output = myprint(&wahl_structure, 0, blank);
    println!("{}", html_output);
    
    println!("</div>");
    
    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cmp_before() {
        let (is_num, val) = cmp_before("123");
        assert!(is_num);
        assert_eq!(val, "123");
        
        let (is_num, val) = cmp_before("abc/123");
        assert!(is_num);
        assert_eq!(val, "123");
        
        let (is_num, val) = cmp_before("abc/def");
        assert!(!is_num);
        assert_eq!(val, "abc/def");
    }
    
    #[test]
    fn test_cmpx() {
        assert_eq!(cmpx("123", "124"), Ordering::Less);
        assert_eq!(cmpx("125", "124"), Ordering::Greater);
        assert_eq!(cmpx("123", "123"), Ordering::Equal);
        
        // Test with "/"
        assert_eq!(cmpx("abc/123", "123"), Ordering::Greater);
        assert_eq!(cmpx("123", "abc/123"), Ordering::Less);
        
        // Test non-numeric
        assert_eq!(cmpx("abc", "def"), Ordering::Less);
        assert_eq!(cmpx("123", "abc"), Ordering::Greater);
    }
    
    #[test]
    fn test_traverse_hierarchy() {
        let liste = vec!["level3".to_string(), "level2".to_string(), "level1".to_string()];
        let value = "item1,item2";
        
        let result = traverse_hierarchy(&liste, OrderedDict::new(), 0, value);
        
        // The result should be a nested structure
        assert!(result.contains_key("level1"));
    }
    
    #[test]
    fn test_merge_dicts() {
        let mut dict1 = OrderedDict::new();
        dict1.insert("key1".to_string(), OrderedValue::String("value1".to_string()));
        
        let mut dict2 = OrderedDict::new();
        dict2.insert("key2".to_string(), OrderedValue::String("value2".to_string()));
        
        merge_dicts(&mut dict1, &dict2);
        
        assert_eq!(dict1.len(), 2);
        assert!(dict1.contains_key("key1"));
        assert!(dict1.contains_key("key2"));
    }
}
// Additional module for handling i18n (simplified version)
mod i18n {
    pub struct ParametersMain;
    
    impl ParametersMain {
        pub const GRUNDSTRUKTUREN: [&'static str; 1] = ["Grundstrukturen"];
    }
}

// Alternative implementation using a simpler approach
fn generate_html_structure_alternative(data: &OrderedDict, blank: bool) -> String {
    let mut html = String::new();
    
    fn build_html_recursive(
        data: &OrderedDict,
        depth: usize,
        blank: bool,
        html: &mut String,
    ) {
        let should_reverse = depth >= 2;
        
        let items: Vec<(&String, &OrderedValue)> = if should_reverse {
            data.iter().rev().collect()
        } else {
            data.iter().collect()
        };
        
        for (key, value) in items {
            let is_container = match value {
                OrderedValue::Dict(dict) => dict.len() > 1 || depth < 2,
                _ => depth < 2,
            };
            
            if is_container {
                html.push_str("<div style=\"white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);\">");
            }
            
            match value {
                OrderedValue::Null => {
                    if blank {
                        html.push_str(&format!(
                            "<input type=\"checkbox\" class=\"ordGru\" onchange=\"toggleP2(this,-10,'✗','{}','{}');\" id=\"ordGru{}\" value=\"{}\">",
                            i18n::ParametersMain::GRUNDSTRUKTUREN[0],
                            key,
                            key,
                            key
                        ));
                    }
                    html.push_str(&format!("<label id=\"ordGruB{}\">{}</label>", 
                        key, 
                        key.replace("_", " ")
                    ));
                    html.push_str("</input>");
                }
                OrderedValue::Dict(inner_dict) => {
                    html.push_str(key);
                    html.push(' ');
                    build_html_recursive(inner_dict, depth + 1, blank, html);
                }
                OrderedValue::String(s) => {
                    html.push_str(s);
                    html.push(' ');
                }
            }
            
            if is_container {
                html.push_str("</div>");
            }
        }
    }
    
    build_html_recursive(data, 0, blank, &mut html);
    html
}

// Example of how to use the library
fn example_usage() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let blank_mode = args.len() > 1 && args[1] == "blank";
    
    // In practice, you would load wahl15 from a JSON file or other source
    let wahl15_data = OrderedDict::new(); // Load your actual data here
    
    // Process the data structure
    let processed_structure = process_wahl_structure(&wahl15_data);
    
    // Generate HTML
    let html_output = if blank_mode {
        format!(
            "<div style=\"white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);\" id='grundstrukturenDiv'>{}",
            myprint(&processed_structure, 0, true)
        )
    } else {
        format!(
            "<div style=\"white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);\">{}",
            myprint(&processed_structure, 0, false)
        )
    };
    
    println!("{}", html_output);
}
