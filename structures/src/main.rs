#[derive(Debug)]
struct Parameters {
    temperature: f64,
    top_k: i16,
    top_p: f64,
    repetition_penalty: Option<f64>,
    max_new_token: i16,
}

impl Parameters {
    fn new() -> Self {
        Self {
            temperature: 1.0,
            top_k: 0,
            top_p: 1.0,
            repetition_penalty: Some(1.0),
            max_new_token: 500,
        }
    }
}


fn main() {
    let params = Parameters::new();
    // print all information of params
    println!("{:?}", params);
}
