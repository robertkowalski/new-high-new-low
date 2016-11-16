use std::env;
use std::process;

extern crate time;

mod lib;


fn main() {

    let args: Vec<String> = env::args().collect();
    let mut print_json = false;
    if args.len() > 1 {
        match &*args[1] {
            "--help" => print_help(),
            "-h" => print_help(),
            "--json" => print_json = true,
            _ => print_help(),
        }
    }

    let time = time::strftime("%Y-%m-%d %T", &time::now()).unwrap();

    if print_json == false {
        // output some content for better ux before data has loaded
        println!("New High - New Low (3 months): {}", time);
        println!("--------------------------------------------------");
    }

    let html = lib::open_nhnl_web();
    let res = lib::scrape(&html);


    let failed = lib::check_result(res);
    for el in failed.iter() {
        println!("[ERROR] couldn't fetch {:?}", el);
        process::exit(1);
    }

    print_result(res, print_json);
}

fn print_result(res: [(&str, i32); 4], print_json: bool) {

    let high = res[0].1 + res[1].1;
    let low = res[2].1 + res[3].1;

    if print_json == false {
        println!("High / Low: {}/{}", high, low);
        println!("");
        println!("τῇ καλλίστῃ.");
        println!("");
    } else {
        let time = time::get_time().sec;
        let res = format!("\"time\": {}, \"high:\": {}, \"low\": {}", time, high, low);
        println!("{{{}}}", res);
    }
}

fn print_help() {

    println!("");
    println!("Usage: nhnl [--help | --json]");
    println!("");
    println!("--json         print result as json");
    println!("--help         print help");
    process::exit(0);
}
