pub enum SimpleStatistics {
    Mean,
    Median,
}

impl SimpleStatistics {
    pub fn calculate<T>(&self, list: impl AsRef<[T]>) -> f64
    where
        T: Into<f64> + Copy,
    {
        match self {
            SimpleStatistics::Mean => SimpleStatistics::mean(list),
            SimpleStatistics::Median => SimpleStatistics::median(list),
        }
    }

    pub fn mean<T>(list: impl AsRef<[T]>) -> f64
    where
        T: Into<f64> + Copy,
    {
        let list = list.as_ref();
        let length = list.len() as f64;
        let total: f64 = list.iter().map(|&i| i.into()).sum();
        total / length
    }

    pub fn median<T>(list: impl AsRef<[T]>) -> f64
    where
        T: Into<f64> + Copy,
    {
        let list = list.as_ref();
        if list.is_empty() {
            panic!("the list is empty")
        }
        let mut sorted: Vec<_> = list.iter().cloned().map(|x| x.into()).collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // if even number of elements
        if list.len() % 2 == 0 {
            let m_idx1 = sorted.len() / 2 - 1;
            let m_idx2 = sorted.len() / 2;
            let median = (sorted[m_idx1] + sorted[m_idx2]) / 2.0;
            median
        } else {
            let m_idx = sorted.len() / 2;
            let median = sorted[m_idx];
            median
        }
    }
}
