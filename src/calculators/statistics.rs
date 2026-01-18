use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Statistics Calculators ---");
        println!("1. Mean, Median, Mode, Range");
        println!("2. Standard Deviation & Variance");
        println!("3. Z-Score Calculator");
        println!("4. Probability Calculator (A and B)");
        println!("5. Permutations & Combinations");
        println!("6. Confidence Interval (95%)");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => mean_median_mode(),
            2 => std_dev_calc(),
            3 => z_score_calc(),
            4 => probability_calc(),
            5 => perm_comb_calc(),
            6 => confidence_interval(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn mean_median_mode() {
    println!("\n--- Mean, Median, Mode, Range ---");
    let count = read_input("How many numbers? ");
    let mut nums = Vec::new();
    for i in 0..(count as usize) {
        nums.push(read_input(&format!("Num {}: ", i + 1)));
    }
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mean: f64 = nums.iter().sum::<f64>() / count;
    let median = if (count as usize) % 2 == 0 {
        (nums[(count as usize)/2 - 1] + nums[(count as usize)/2]) / 2.0
    } else {
        nums[(count as usize)/2]
    };
    let range = nums.last().unwrap() - nums.first().unwrap();

    println!("Mean: {:.2}", mean);
    println!("Median: {:.2}", median);
    println!("Range: {:.2}", range);
    println!("(Mode is complex for CLI, check sorted list: {:?})", nums);
}

fn std_dev_calc() {
    println!("\n--- Standard Deviation ---");
    let count = read_input("How many numbers? ");
    let mut nums = Vec::new();
    for i in 0..(count as usize) {
        nums.push(read_input(&format!("Num {}: ", i + 1)));
    }
    let mean: f64 = nums.iter().sum::<f64>() / count;
    let variance: f64 = nums.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / count;
    println!("Variance: {:.2}", variance);
    println!("Standard Deviation: {:.2}", variance.sqrt());
}

fn z_score_calc() {
    let x = read_input("Value (x): ");
    let mu = read_input("Mean (μ): ");
    let sigma = read_input("Std Dev (σ): ");
    println!("Z-score: {:.2}", (x - mu) / sigma);
}

fn probability_calc() {
    let pa = read_input("P(A): ");
    let pb = read_input("P(B): ");
    println!("P(A and B) [Independent]: {:.4}", pa * pb);
    println!("P(A or B) [Independent]: {:.4}", pa + pb - (pa * pb));
}

fn perm_comb_calc() {
    let n = read_input("n: ") as u64;
    let r = read_input("r: ") as u64;
    
    fn fact(num: u64) -> f64 {
        (1..=num).map(|x| x as f64).product()
    }

    let p = fact(n) / fact(n - r);
    let c = p / fact(r);
    println!("Permutations (nPr): {:.0}", p);
    println!("Combinations (nCr): {:.0}", c);
}

fn confidence_interval() {
    let mean = read_input("Mean: ");
    let std_dev = read_input("Std Dev: ");
    let n = read_input("Sample Size: ");
    let margin = 1.96 * (std_dev / n.sqrt());
    println!("95% Confidence Interval: {:.2} ± {:.2}", mean, margin);
}
