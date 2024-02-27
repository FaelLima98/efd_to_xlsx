use std::fs::File;
use std::io::{prelude::*, BufReader};

use rust_xlsxwriter::{Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let file = File::open("foo.txt").expect("Erro ao encontrar o arquivo");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let mut line_counter: u32 = 0;

    for line in lines {

        let mut value_counter: u16 = 0;

        for value in line.split("|") {

            if value.is_empty() {
                continue;
            }

            worksheet.write(line_counter, value_counter, value)?;

            value_counter += 1;
        }

        line_counter += 1;
    }

    workbook.save("demo.xlsx")?;

    Ok(())
}