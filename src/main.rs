use std::collections::HashMap;
use std::fs;
use std::{env, process};

enum Item {
    Num(f64),
    Str(String),
}

fn print_item(item: &Item) {
    match item {
        Item::Num(n) => print!("\x1b[34m{}\x1b[0m", n), //blue
        Item::Str(s) => print!("\x1b[31m{}\x1b[0m", s), //red
    }
}

struct CsvFile {
    items: Vec<Vec<Item>>,
    columns_to_idx: HashMap<String, usize>,
    column_data_types: HashMap<String, bool>, //true = num
}

impl CsvFile {
    fn new(csv_str: &str) -> Self {
        let rows: Vec<&str> = csv_str.lines().collect();

        let headers: Vec<String> = rows[0].split(',').map(|s| s.trim().to_string()).collect();

        let mut columns_to_idx = HashMap::new();
        for (idx, header) in headers.iter().enumerate() {
            columns_to_idx.insert(header.clone(), idx);
        }

        let mut column_data_types: HashMap<String, bool> = HashMap::new();
        for h in &headers {
            column_data_types.insert(h.clone(), true);
        }

        let mut items = Vec::new();
        //skipping the header
        for row in &rows[1..] {
            let row_data: Vec<Item> = row
                .split(',')
                .enumerate()
                .map(|(col_idx, cell)| {
                    let prepped_str = cell.trim().to_string();
                    match prepped_str.parse::<f64>() {
                        Ok(x) => Item::Num(x),
                        Err(_) => {
                            column_data_types.insert(headers[col_idx].clone(), false);
                            Item::Str(prepped_str)
                        }
                    }
                })
                .collect();
            items.push(row_data);
        }

        CsvFile {
            items,
            columns_to_idx,
            column_data_types,
        }
    }

    fn print_headers(&self) {
        let mut headers_ordered = vec![""; self.columns_to_idx.len()];
        for (name, &idx) in &self.columns_to_idx {
            headers_ordered[idx] = name;
        }

        for (i, header) in headers_ordered.iter().enumerate() {
            if i > 0 {
                print!(",");
            }
            print!("{}", header);
        }
        println!()
    }

    fn avg(&self, column: &str) -> Result<f64, String> {
        if !self.columns_to_idx.contains_key(column) {
            return Err(format!("Column '{}' not found", column));
        }

        if !self.column_data_types[column] {
            return Err(format!("Column '{}' is not numeric", column));
        }

        let idx = self.columns_to_idx[column];

        if self.items.is_empty() {
            return Err("no data in the file".to_string());
        }

        let mut sum = match &self.items[0][idx] {
            Item::Num(x) => *x,
            Item::Str(_) => return Err("expected number, found string".to_string()),
        };

        for i in 1..self.items.len() {
            match &self.items[i][idx] {
                Item::Num(x) => sum += x,
                Item::Str(_) => return Err("expected number, found string".to_string()),
            }
        }

        Ok(sum / self.items.len() as f64)
    }
    fn min(&self, column: &str) -> Result<f64, String> {
        if !self.columns_to_idx.contains_key(column) {
            return Err(format!("Column '{}' not found", column));
        }

        if !self.column_data_types[column] {
            return Err(format!("Column '{}' is not numeric", column));
        }

        let idx = self.columns_to_idx[column];

        if self.items.is_empty() {
            return Err("no data in the file".to_string());
        }

        let mut min = match &self.items[0][idx] {
            Item::Num(x) => *x,
            Item::Str(_) => return Err("expected number but found string".to_string()),
        };

        for i in 1..self.items.len() {
            match &self.items[i][idx] {
                Item::Num(x) => min = min.min(*x),
                Item::Str(_) => return Err("expected number but found string".to_string()),
            }
        }

        Ok(min)
    }
    fn max(&self, column: &str) -> Result<f64, String> {
        if !self.columns_to_idx.contains_key(column) {
            return Err(format!("Column '{}' not found", column));
        }

        if !self.column_data_types[column] {
            return Err(format!("Column '{}' is not numeric", column));
        }

        let idx = self.columns_to_idx[column];

        if self.items.is_empty() {
            return Err("no data in the file".to_string());
        }

        let mut max = match &self.items[0][idx] {
            Item::Num(x) => *x,
            Item::Str(_) => return Err("expected number, found string".to_string()),
        };

        for i in 1..self.items.len() {
            match &self.items[i][idx] {
                Item::Num(x) => max = max.max(*x),
                Item::Str(_) => return Err("expected number, found string".to_string()),
            }
        }

        Ok(max)
    }
    fn head(&self, n: usize) {
        if n > self.items.len() {
            println!("there are only {} rows in the file", self.items.len());
            process::exit(1);
        }

        self.print_headers();

        for i in 0..n.min(self.items.len()) {
            for j in &self.items[i] {
                print_item(j);
                print!(",");
            }
            println!();
        }
    }
    fn tail(&self, n: usize) {
        if n > self.items.len() {
            println!("there are only {} rows in the file", self.items.len());
            process::exit(1);
        }

        self.print_headers();

        for i in (self.items.len() - n).max(0)..(self.items.len()) {
            for j in &self.items[i] {
                print_item(j);
                print!(",");
            }
            println!();
        }
    }
    fn rows(&self) {
        println!("Number of rows: {}", self.items.len());
    }
    //pretty print for the whole file
    fn show(&self) {
        self.print_headers();

        for row in &self.items {
            for i in row {
                print_item(i);
                print!(",")
            }
            println!();
        }
    }
    fn show_column(&self, col: &String) {
        println!("{}", col);

        if !self.columns_to_idx.contains_key(col) {
            panic!("Column '{}' not found", col);
        }

        if self.items.is_empty() {
            panic!("no data in the file");
        }
        let idx = self.columns_to_idx[col];

        for row in &self.items {
            print_item(&row[idx]);
            println!();
        }
    }
    fn columns(&self) {
        println!("Columns: ");
        for (name, &is_num) in &self.column_data_types {
            let type_str = if is_num { "number" } else { "text" };
            println!("  {} ({})", name, type_str);
        }
    }
    fn unique(&self, column: &String) {
        if !self.columns_to_idx.contains_key(column) {
            panic!("Column '{}' not found", column);
        }

        if self.items.is_empty() {
            panic!("no data in the file");
        }

        let idx = self.columns_to_idx[column];

        let mut map: HashMap<String, usize> = HashMap::new();

        for row in &self.items {
            match &row[idx] {
                Item::Num(x) => *map.entry(x.to_string()).or_insert(0) += 1,
                Item::Str(s) => *map.entry(s.clone()).or_insert(0) += 1,
            }
        }

        println!("Column '{}' has {} unique elements:", column, map.len());

        for (key, val) in &map {
            println!("{} : {}", key, val);
        }
    }
}

fn print_help() {
    println!(
        "CSV Analyzer - A command-line tool for analyzing CSV files

USAGE:
    csv_tool <file.csv> <command> [argument]
    csv_tool help

COMMANDS:
    columns              Show all columns and their data types
    rows                 Show total number of rows
    show                 Display the entire CSV file with colored output
    show <column>        Displays the given column
    head [n]             Show first n rows (default: all rows)
    tail [n]             Show last n rows (default: all rows)
    avg <column>         Calculate average of numeric column
    min <column>         Find minimum value in numeric column
    max <column>         Find maximum value in numeric column
    unique <column>      Show unique values and their counts in column

EXAMPLES:
    csv_tool data.csv columns
    csv_tool data.csv head 10
    csv_tool data.csv avg salary
    csv_tool data.csv unique department"
    );
}

//args[1]: file, args[2]: command, args[3] argument for the command
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "help" {
        print_help();
        return;
    }

    if args.len() > 4 {
        panic!("too many args!");
    }

    if args.len() < 3 {
        panic!("too few args!");
    }

    let command = &args[2];

    let csv_text = fs::read_to_string(&args[1]);

    let content = match csv_text {
        Ok(s) => s,
        Err(e) => {
            println!("Error reading file: '{}', {}", args[1], e);
            process::exit(1);
        }
    };

    let csv_file = CsvFile::new(&content);

    match command.as_str() {
        "columns" => csv_file.columns(),
        "rows" => csv_file.rows(),
        "head" => {
            let n = if args.len() > 3 {
                args[3].parse().unwrap_or(csv_file.items.len())
            } else {
                csv_file.items.len()
            };
            csv_file.head(n);
        }
        "tail" => {
            let n = if args.len() > 3 {
                args[3].parse().unwrap_or(csv_file.items.len())
            } else {
                csv_file.items.len()
            };
            csv_file.tail(n);
        }
        "show" => {
            if args.len() == 4 {
                csv_file.show_column(&args[3]);
            } else {
                csv_file.show()
            }
        }
        "avg" => {
            if args.len() < 4 {
                println!("usage: avg <column name>");
                return;
            }
            match csv_file.avg(&args[3]) {
                Ok(res) => println!("Average: {:.2}", res),
                Err(e) => println!("Error: {}", e),
            }
        }
        "min" => {
            if args.len() < 4 {
                println!("usage: min <column name>");
                return;
            }
            match csv_file.min(&args[3]) {
                Ok(res) => println!("Min: {}", res),
                Err(e) => println!("Error: {}", e),
            }
        }
        "max" => {
            if args.len() < 4 {
                println!("usage: max <column name>");
                return;
            }
            match csv_file.max(&args[3]) {
                Ok(res) => println!("Max: {}", res),
                Err(e) => println!("Error: {}", e),
            }
        }
        "unique" => {
            if args.len() < 4 {
                println!("usage: unique <column name>");
                return;
            }
            csv_file.unique(&args[3]);
        }
        _ => println!("unknown command: {}", command),
    }
}
