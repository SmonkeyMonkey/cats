extern crate rusty_machine;

use std::error::Error;

use rusty_machine::{
    learning::k_means::KMeansClassifier,
    prelude::{BaseMatrix, Matrix, UnSupModel},
};

const CLUSTER_COUNT: usize = 3;

fn main() {
    let samples = read_data_from_stdin().unwrap();

    let mut model = KMeansClassifier::new(CLUSTER_COUNT);
    model.train(&samples).unwrap();

    let classes = model.predict(&samples).unwrap();

    export_results_to_stdout(samples, classes.into_vec()).unwrap();
}

fn read_data_from_stdin() -> Result<Matrix<f64>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(std::io::stdin());
    let mut data: Vec<f64> = vec![];

    for result in reader.records() {
        let record = result?;
        data.push(record[0].parse().unwrap());
        data.push(record[1].parse().unwrap());
    }
    Ok(Matrix::new(&data.len() / 2, 2, data))
}

fn export_results_to_stdout(
    samples: Matrix<f64>,
    classes: Vec<usize>,
) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    writer.write_record(&["height", "length", "class"]);

    for sample in samples.iter_rows().zip(classes) {
        writer.serialize(sample)?;
    }

    Ok(())
}
