use std::io;

pub fn calculate_bmi(){
    println!("=================");
    println!("BMI Calculator");
    println!("--------------");
    println!("Healthy BMI range: 18.5 kg/m2 - 25 kg/m2 \nHealthy weight for the height: 59.9 kgs - 81.0 kgs \nPonderal Index: 11.1 kg/m3");
    println!("=================");
    
    //let mut age = String::new();
    //let mut gender = String::new();
    let mut height = String::new();
    let mut weight = String::new();

    // println!("Please Enter your Age \n [Ages between: 2 - 120");
    // io::stdin().read_line(&mut age).unwrap();
    // let converted_age : i32 = age.trim().parse().unwrap();
    
    // println!("Please Select Gender. \n 1.Male \n 2.Female");
    // io::stdin().read_line(&mut gender).unwrap();
    // let converted_gender : i32 = gender.trim().parse().unwrap();
    
    println!("Please Enter your Height in cm");
    io::stdin().read_line(&mut height).unwrap();
    let converted_height : f32 = height.trim().parse().unwrap();
    
    println!("Please Enter your Weight in kg");
    io::stdin().read_line(&mut weight).unwrap();
    let converted_weight : f32 = weight.trim().parse().unwrap();

    let bmi_result = bmi(converted_weight, converted_height);
    println!("{}", bmi_result);

    fn bmi(converted_weight: f32, converted_height: f32) -> &'static str {
        
        let index = converted_weight as f32 / converted_height.powi(2);
        
        match index {
            index if index <= 18.5 => "Underweight",
            index if index <= 25.0 => "Normal",
            index if index <= 30.0 => "Overweight",
            _ => "Obese"
        }
    }
}