use clap::Parser;
use proportional_cost_splitter_lib::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///individual costs before vats, service charges etc
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    costs: Vec<f64>,

    ///final total after vats and charges
    #[arg(short, long)]
    total: f64,
}

fn main() {
    let args = Args::parse();

    let scaled_costs = 
        scale_to_total(
            args.costs
                .into_iter()
                .map(|cost|(None, cost))
                .collect::<Vec<_>>(), args.total);

    let outputs = 
        scaled_costs
            .iter()
            .map(|(cost, scaled)| format!("{cost} => {:.2}", scaled))
            .collect::<Vec<_>>();

    println!("{:?}", outputs);
}