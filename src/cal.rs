use chrono::Datelike;

pub fn print_cal_for_month(date: crate::LocalDate, prune: bool) {
    let weeks = crate::weeks_for_month(date, prune);

    println!("{:^27}", date.format("%b %Y"));

    // Use the second week, since it will always have 7 days.
    for day in &weeks[1] {
        match day {
            None => print!("    "),
            Some(day) => print!("{}", day.date.format("%a ")),
        }
    }
    println!();

    for week in &weeks {
        for day in week {
            match day {
                None => print!("    "),
                Some(d) => print!("{:3} ", d.date.day()),
            }
        }
        println!();
    }
}
