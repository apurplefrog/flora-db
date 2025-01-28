/*
 * A: Atomic
 * C: Consistency (irrelevant for database creation)
 * I: Isolation (no dirty reads / writes)
 * D: Durability (stored on disk)
 */

use std::collections::HashMap;

#[derive(Debug)]
struct Database {
    tables: Option<HashMap<String, Table>>,
}

impl Database {
    fn new() -> Self {
        Self { tables: None }
    }

    fn add_table(&mut self, name: String) {
        self.tables = Some(HashMap::from([(name, Table {
            rows: None,
            cols: None,
            data: None,
        })]));
    }

    fn get_mut_tables(&mut self) -> &mut Option<HashMap<String, Table>> {
        &mut self.tables
    }
}

#[derive(Debug)]
struct Table {
    rows: Option<Vec<String>>,
    cols: Option<Vec<String>>,
    data: Option<Vec<Vec<Cell>>>,
}

impl Table {
    fn add_row(&mut self, row_name: String) {
        self.rows.as_mut().unwrap().push(row_name);
    }
}

#[derive(Debug)]
enum Cell {
    String(String),
    Integer(i32),
    Float(f32),
    None,
}

fn main() {
    // Create database
    let mut database = Database::new();

    // Add table People
    database.add_table("People".into());

    // Add columns [name, age, id]
    let mut tables = database.get_mut_tables().as_mut().unwrap();
    tables.insert("People".into(), Table {
        rows: Some(vec!["name".into(), "age".into(), "id".into()]),
        cols: None,
        data: None,
    });

    // Add John to people with age 27 and id 0
    // Add Mary to people with age 28 and id 1
    // Get John's age
    // Set John's age to 28
    // Get John's age (should err)
    // Commit John's age
    // Get John's age
}
