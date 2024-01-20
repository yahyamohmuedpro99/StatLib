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

// make the function accept arr or vec bcz i will use it in multiple things later 
pub fn Mean<T>(list:impl AsRef<[T]>)->f64
    where T:Into<f64>+Copy{
        let list=list.as_ref();
        let length=list.len() as f64;
        let total = &list.iter().map(|&i| i.into()).sum();
        total/length
    }

pub fn Median<T>(list:&[T])->f64
    where T:Into<f64>+Copy{
        if list.is_empty(){
            panic!("the list is empty")
        }
        let mut sorted: Vec<_> = list.iter().cloned().map(|x| x.into()).collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Sort the cloned elements
        // if even number of elements
        if list.len()%2 == 0{
            let m_idx1=sorted.len()/2 - 1;
            let m_idx2=sorted.len()/2;
            let medien=(sorted[m_idx1]+sorted[m_idx2])/2.0;
            medien
        }else{
            let m_idx=sorted.len()/2;
            let medien=sorted[m_idx];
            medien
        }
        
    }