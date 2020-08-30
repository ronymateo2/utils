mod utils;

extern crate lazy_static;
use std::sync::Mutex;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref USERS: Mutex<HashMap<i32, String>> = {
        let mut map = HashMap::new();
        map.insert(1, String::from("admin"));
        map.insert(2, String::from("user"));
        Mutex::new(map)
    };
}

#[wasm_bindgen]
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Issue {
    Feature = 0,
    BugFix = 1,
}

#[wasm_bindgen]
pub struct Utils {}

#[wasm_bindgen]
impl Utils {
    pub fn dashed(&self, issue_name: &str, issue: Issue) -> String {
        let reg_issue_code = Regex::new(r"#\d+$").unwrap();
        let issue_description = reg_issue_code.replace(issue_name, "");

        let dashed = issue_description.trim().replace(" ", "-");

        let reg_issue_number = Regex::new(r"#(?P<m>\d+)$").unwrap();

        if reg_issue_number.is_match(issue_name) {
            let caps = reg_issue_number.captures(issue_name).unwrap();
            let issue_number = caps.get(1).map_or("", |m| m.as_str());
            match issue {
                Issue::BugFix => format!("Bugfix/{}-{}", issue_number, dashed),
                Issue::Feature => format!("Feature/{}-{}", issue_number, dashed),
            }
        } else {
            match issue {
                Issue::BugFix => format!("Bugfix/{}", dashed),
                Issue::Feature => format!("Feature/{}", dashed),
            }
        }
    }

    pub fn get(&self, index: i32) -> String {
        let map = USERS.lock().unwrap();
        let value = map.get(&index).unwrap();
        value.to_string()
    }
    pub fn add(&self, key: i32, code: String) {
        let g = USERS.lock();
        let mut map = g.unwrap();
        map.insert(key, code);
    }

    pub fn new() -> Utils {
        Utils {}
    }
}
