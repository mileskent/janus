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

#[derive(Debug)]
struct Event {
    start_date: Date,
    end_date: Date,
    title: String,
    description: String
}


#[derive(Debug)]
struct Task {
    due_date: Date,
    title: String,
    description: String
}

fn main() {
}
