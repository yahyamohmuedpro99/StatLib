// - this will be my simple stats module

// - Beginner (Daily Use):
/*
    - Mean (Average): Sum of values divided by the number of values.
    - Median: Middle value of a dataset when it's ordered.
    - Mode: Most frequently occurring value in a dataset.
    - Range: Difference between the maximum and minimum values.
    - Count: Total number of observations in a dataset.
    - Percentage: Portion of a whole expressed as a percentage.
    - Sum: Total of all values in a dataset.
    - Min and Max: Identify the smallest and largest values in a dataset.

*/

mod advanced;
mod beginner;
mod inter;

use beginner::SimpleStatistics;

fn main() {
    let stats = SimpleStatistics::Mean;
    let data = [6.5, 1.1, 2.1, 3.5];
    let result = stats.calculate(&data);
    println!("Mean: {}", result);

    let stats = SimpleStatistics::Median;
    let data = vec![1, 2, 3, 4, 5];
    let result = stats.calculate(&data);
    println!("Median: {}", result);
}
