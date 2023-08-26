#[derive(Debug)]
struct Parameters {
    temperature: f64,
    top_k: i16,
    top_p: f64,
    repetition_penalty: Option<f64>,
    max_new_token: i16,
}
//Contructor with default values
impl Parameters {
    fn new(
        temperature: f64,
        top_k: i16,
        top_p: f64,
        repetition_penalty: Option<f64>,
        max_new_token: i16,
    ) -> Self {
        Parameters {
            temperature,
            top_k,
            top_p,
            repetition_penalty,
            max_new_token,
        }
    }
}
    
fn main() {
    let params: Parameters = Parameters::new(1.0, 10, 0.9, Some(2.0), 10);
    // print all information of params
    println!("{:?}", params);
}
