#[derive(Debug)]
struct Time {
    hour: u8, 
    minute: u8
}

#[derive(Debug)]
struct Date {
    year: u16,
    month: u8,
    time: Time
}

impl Date {
    fn month_string (&self) -> &str {
        let months: [&str; 12] = [
            "January", "February", "March", "April",
            "May", "June", "July", "August", "September",
            "October", "November", "December"
        ];
        match months.get((self.month as usize) - 1) {
            Some (month) => month,
            None => "Invalid month",
        }
    }
}

struct Event {
    date: Date,
    title: String,
    description: String
}

fn main() {
    let t: Time = Time {hour: 10, minute: 30};
    println!("{:#?}", t);
    let d: Date = Date {year: 2024, month: 7, time: t};
    println!("{:#?}", d);
    println!("{}", d.month_string());
}
