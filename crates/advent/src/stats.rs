use std::time::Duration;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Stats {
    pub samples: usize,
    pub min: Duration,
    pub max: Duration,
    pub mean: Duration,
    pub std_dev: Duration,
}

pub fn basics(data: &[Duration]) -> Stats {
    let mut data: Vec<f64> = data.iter().map(|d| d.as_secs_f64()).collect();
    data.sort_by(f64::total_cmp);

    // remove extreme outliers 🤷‍♂️
    if data.len() > 1_000 {
        let min = percentile(&data, 1.0);
        let max = percentile(&data, 99.0);
        data.retain(|&t| t >= min && t <= max);
    }

    let len = data.len();
    let min = data[0];
    let max = data[len - 1];
    let mean = {
        let sum: f64 = data.iter().sum();
        sum / (len as f64)
    };
    let std_dev = {
        let sum: f64 = data
            .iter()
            .map(|x| {
                let y = x - mean;
                y * y
            })
            .sum();
        let variance = sum / (len - 1) as f64;
        variance.sqrt()
    };
    Stats {
        samples: len,
        min: Duration::from_secs_f64(min),
        max: Duration::from_secs_f64(max),
        mean: Duration::from_secs_f64(mean),
        std_dev: Duration::from_secs_f64(std_dev),
    }
}

pub fn percentile(data: &[f64], pct: f64) -> f64 {
    let zero: f64 = 0.0;
    let hundred: f64 = 100.0;
    assert!(zero <= pct);
    assert!(pct <= hundred);

    if (pct - hundred).abs() < f64::EPSILON {
        return data[data.len() - 1];
    } else if pct == 0.0 {
        return data[0];
    }

    let len = (data.len() - 1) as f64;
    let rank = (pct / hundred) * len;
    let lrank = rank.floor();
    let d = rank - lrank;
    let n = lrank as usize;
    let lo = data[n];
    let hi = data[n + 1];
    lo + (hi - lo) * d
}
