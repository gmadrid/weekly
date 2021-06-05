use argh::FromArgs;
use chrono::Local;

#[derive(FromArgs)]
/// Spew a calendar.
struct Args {
    #[argh(switch)]
    /// prune any dates not in the requested month.
    prune: bool,
}

fn main() {
    let args: Args = argh::from_env();
    
    weekly::print_cal_for_month(Local::now().date(), args.prune);
}
