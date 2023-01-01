use neuroflow::{
    FeedForward,
    data::DataSet,
    activators::Type::Tanh,
    io
};
use rand::Rng;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rand_generator = rand::thread_rng();

    let mut neural_net = FeedForward::new(&[1, 7, 8, 8, 7, 1]);

    let mut data = DataSet::new();

    for _x in 1..15000 {
        let val1: f64 = rand_generator.gen();
        let val2: f64 = rand_generator.gen();
        data.push(&[val1], &[val2]);
    }

    neural_net.activation(Tanh).learning_rate(0.01).train(&data, 5000);

    let new_val: f64 = rand_generator.gen();

    let check_val = neural_net.calc(&[new_val])[0];
    println!("Calculated value: {}", check_val);

    io::save(&neural_net, "fakecorrelation.flow").unwrap();

    Ok(())
}
