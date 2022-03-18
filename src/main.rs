use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let year: i32 = rng.gen_range(0, 9999);
    let month_raw: i32 = rng.gen_range(1, 12);
    let mut day: i32 = 0;
    if month_raw == 2 {
        if year % 4 == 0 {
                if year % 100 == 0 && year % 400 != 0 {
                    day = rng.gen_range(1, 28);
                } else {
                    day = rng.gen_range(1, 29);
                }
            }
        } else if month_raw == 4 || month_raw == 6 || month_raw == 9 || month_raw == 11 {
            day = rng.gen_range(1, 30);
        } else {
            day = rng.gen_range(1, 31);
        }
    let hour: i32  = rng.gen_range(0, 23);
    let min: i32 = rng.gen_range(0, 59);
    let sec: i32 = rng.gen_range(0, 59);
    
    let year_tmp: i32;
    let month_tmp: i32;
    if month_raw < 3 {
        year_tmp = year - 1;
        month_tmp = month_raw + 12;
    } else {
        year_tmp = year;
        month_tmp = month_raw;
    }
    let week_raw: i32 = (year_tmp + year_tmp / 4 - year_tmp / 100 + year_tmp / 400 + (13 * month_tmp + 8) / 5 + day) % 7;
    
    let mut week = "N/A";
    match week_raw {
        0 => week = "Sun",
        1 => week = "Mon",
        2 => week = "Tue",
        3 => week = "Wed",
        4 => week = "Thu",
        5 => week = "Fri",
        6 => week = "Sat",
        _ => println!("WTF")
    }
    
    let mut month = "N/A";
    match month_raw {
        1 => month = "Jan",
        2 => month = "Feb",
        3 => month = "Mar",
        4 => month = "Apr",
        5 => month = "May",
        6 => month = "Jun",
        7 => month = "Jul",
        8 => month = "Aug",
        9 => month = "Sep",
        10 => month = "Oct",
        11 => month = "Nov",
        12 => month = "Dec",
        _ => println!("WTF")
    }
    
    println!("{} {} {} {}:{}:{} YST {}\n", week, month, day, hour, min, sec, year);
}
