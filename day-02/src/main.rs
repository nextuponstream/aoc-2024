use day_02::Report;

fn main() {
    let input = std::fs::read_to_string("./day-02/input").unwrap();

    let lines: Vec<&str> = input.split("\n").collect();
    let reports: Vec<Report> = lines.iter().map(|line| (*line).try_into().unwrap()).collect();
    let safe_count = reports.iter().filter(|r| r.safe()).count();
    println!("{safe_count}")
}