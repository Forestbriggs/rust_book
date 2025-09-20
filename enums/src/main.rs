fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let other row = vec![
        SpreadsheetCell::Int(4),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(12.47),
    ]
}
