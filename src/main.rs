/*
 * A: Atomic
 * C: Consistency (irrelevant for database creation)
 * I: Isolation (no dirty reads / writes)
 * D: Durability (stored on disk)
 */

struct Database {
    changes: Vec<DatabaseChange>,
}

impl Database {
    fn new() -> Self {
        Self { changes: vec![] }
    }
    fn create_table(&mut self, name: TableName) {
        todo!();
    }
    fn commit(&mut self) {
        todo!();
    }
}

struct TableName(String);

enum DatabaseChange {
    CreateTable(TableName),
    RemoveTable(TableName),
    EditTable(TableName, Vec<TableChange>),
}

enum CellType {
    String(String),
    Integer(i64),
    Float(f64),
    None,
}

struct ColumnName;

struct RowName;
struct Row {
    contents: Vec<CellType>,
}

enum TableChange {
    AddColumn(ColumnName),
    RemoveColumn(ColumnName),
    AddRow(RowName, Row),
    RemoveRow(RowName, Row),
    EditCell(ColumnName, RowName, CellType),
}

fn main() {
    let mut database = Database::new();
    database.create_table(TableName("YAY".into()));
    database.commit();
}
