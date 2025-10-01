const COLS: i32 = 8;

fn cell_name(index: i32) -> (i32, char, String) {
    let row = index / COLS + 1;
    let cno: u8 = (index % COLS + 65).try_into().unwrap();
    let colname: char = cno as char;
    let cellname = format!("{}{}", colname, row);

    (row, colname,  cellname)

}

fn main() {
    // println!("Welcome to exc");

    let mut sheet = Sheet::new();
    for i in 0..24 {
        if i % 8 == 0 {
            println!("");
        }
        let cell = {
            let (row, _col, _cellname, ) = cell_name(i);

            if i % COLS == 0 {
                Cell::new(format!("Row {row}",).as_str())
            } else if i % COLS == 7 {
                let cell_alfa = sheet.find_cell("B", row).unwrap();
                let cell_beta = sheet.find_cell("C", row).unwrap();
                let operator = "-";
                let raw_value = format!("C{row}{operator}B{row}");
                Cell::new_formula(cell_alfa, operator, cell_beta, raw_value)
                //Example B1 + C1
            } else {
                Cell::new_num(22.0/7.0 * i as f32)
            }
        };

        sheet.add(cell);
    }

    print!("RENDERING IN RAW MODE");
    sheet.print();
    sheet.solve();

    print!("\n\nRENDERING IN SOLVE MODE");
    sheet.print();
}

#[derive(Clone)]
struct SimpleFormula {
    alfa: Box<Cell>, 
    beta: Box<Cell>,
    operator: String,
    label: String
}

impl SimpleFormula {
    fn from(alfa: Cell, ope: &str, beta: Cell, label: String) -> Self {
        Self {
            alfa: Box::new(alfa),
            beta: Box::new(beta),
            operator: format!("{ope}"),
            label: label
        }
    }

    fn raw(self) -> String {
    //    let alfa = *self.alfa;
    //    let beta = *self.beta;
       format!("{label}", label = self.label)   
    }

    fn solve(alfa: Cell, beta: Cell, operator: String) -> String {
        match operator.as_str() {
            "+" => SumCell(alfa, beta),
            "-" => SubCell(alfa, beta),
            "*" => MulCell(alfa, beta),
            "/" => DivCell(alfa, beta),
            "//" => RemCell(alfa, beta),
            _ => "⏳".to_string()
        }
    }
}

fn SumCell(alfa: Cell, beta: Cell) -> String {
    match (alfa, beta) {
        (Cell::Num(a), Cell::Num(b)) => format!("{:8.2}", a+b), 
        (Cell::Num(a), Cell::Value(b)) => format!("{a} {b}"), 
        (Cell::Value(a), Cell::Num(b)) => format!("{a} {b}"), 
        (Cell::Value(a), Cell::Value(b)) => format!("{a} {b}"), 
        _ => format!("⏳")
    }
}
fn SubCell(alfa: Cell, beta: Cell) -> String {
    match (alfa, beta) {
        (Cell::Num(a), Cell::Num(b)) => format!("{:8.2}", a-b), 
        (Cell::Num(a), Cell::Value(b)) => format!("{a} {b}"), 
        (Cell::Value(a), Cell::Num(b)) => format!("{a} {b}"), 
        (Cell::Value(a), Cell::Value(b)) => format!("{a} {b}"), 
        _ => format!("⏳")
    }
}

fn MulCell(alfa: Cell, beta: Cell) -> String {
    match (alfa, beta) {
        (Cell::Num(a), Cell::Num(b)) => format!("{:8.2}", a*b), 
        _ => format!("⏳")
    }
}

fn DivCell(alfa: Cell, beta: Cell) -> String {
    match (alfa, beta) {
        (Cell::Num(a), Cell::Num(b)) => {
            if b == 0.0 {
                format!("{:8}", "DIV/0")
            } else {
                format!("{:8.2}", a/b)
            }
        }, 
        _ => format!("⏳")
    }
}

fn RemCell(alfa: Cell, beta: Cell) -> String {
    match (alfa, beta) {
        (Cell::Num(a), Cell::Num(b)) => {
            if b == 0.0 {
                format!("{:8}", "DIV/0")
            } else {
                format!("{:8.2}", a%b)
            }
        }, 
        _ => format!("⏳")
    }
}

#[derive(Clone)]
enum Cell {
    Value(String),
    Num(f32),
    Formula(SimpleFormula),
}

#[derive(Clone)]
enum SheetMode {
    Raw,
    Solved,
}

#[derive(Clone)]
struct Sheet {
    cells: Vec<Cell>,
    mode: SheetMode,
}

impl Sheet {
    fn new() -> Sheet {
        Sheet {
            cells: vec!(),
            mode: SheetMode::Raw
        }
    }

    fn solve(&mut self) {
        self.mode = SheetMode::Solved;
    }

    fn head(&self) {
        print!("\n    |");
        for j in 0..COLS {
            let cno: u8 = (65 + j).try_into().unwrap();
            let c: char = cno as char;
          print!("{c:-8} | ");  
        }
        print!("\n    |");
        for _ in 0..COLS {
          print!("{c:-8} | ", c = "--------");  
        }
    }

    fn add(&mut self, cell: Cell) {
        self.cells.push(cell)
    }

    fn print(&self) {
        self.head();
        for (i, cell) in self.cells.iter().enumerate() {
            cell.print(i as i32, self.mode.clone(), self.clone());
        }
    }

    fn resolve_formula(self, formula: SimpleFormula) -> String{
        // format!("{x:7}", x="📱")
        SimpleFormula::solve(*formula.alfa, *formula.beta, formula.operator)

    }

    fn find_cell(&self, col: &str, row: i32) -> Option<Cell> {
        let coord = format!("{col}{row}");
        
        
        for (index, cell) in self.cells.iter().enumerate() {
            let (row, colname,  cellname) = cell_name(index as i32);
            if cellname == coord {
                return Some(cell.clone())
            }
        }
        return None
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Formula(formula) => write!(f, "{}", formula.label),
            Cell::Value(value) => write!(f, "{}", value),
            Cell::Num(value) => write!(f, "{:.2}", value),

        }        
    }
}

impl Cell {
    fn new(value: &str) -> Cell {
        Cell::Value(value.to_string())
    }
    fn new_num(value: f32) -> Cell {
        Cell::Num(value)
    }

    fn new_formula(alfa: Cell, ope: &str, beta: Cell, label: String) -> Cell {
        Cell::Formula(SimpleFormula::from(alfa, ope, beta, label))
    }

    fn print(&self, index: i32, mode: SheetMode, sheet_data: Sheet) {

        if index % COLS == 0 {
            print!("\n{:3} |", index / COLS + 1);
        }

        let print_formula = |formula: SimpleFormula| {
            match mode {
                SheetMode::Raw => print!("= {f:6} | ", f = formula.raw()),
                _ => print!("{render:7} | ", render = sheet_data.resolve_formula(formula)),
            }
        };

        match self {
            Cell::Value(some_value) => print!("{some_value:-8} | "),
            Cell::Num(some_value) => print!("{some_value:8.2} | "),
            Cell::Formula(some_formula) => print_formula(some_formula.clone()),
        }
    }
}
