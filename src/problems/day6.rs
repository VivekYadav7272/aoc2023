use crate::utils::parse_stuff;

use super::Problem;

fn margin_of_error(t_thres: usize, d: usize) -> usize {
    // eqn for time taken is: t(t_btn, d) = t_btn + d/t_btn (cuz t_btn is now velocity,
    // so don't think d/t == velocity here, it's d/v == time)

    // To solve for t_btn that makes t(t_btn, d) = t_thres, the eqn becomes,
    // t_thres = t_btn + d/t_btn
    // => t_btn^2 - t_thres*t_btn + d = 0
    // => D = sqrt(t_thres^2 - 4*d)
    // => t_btn_min = ceil((t_thres - D)/2)
    // => t_btn_max = floor((t_thres + D)/2)
    // BUUT, remember, the fastest you CAN go is when dt/d(t_btn) = 0.
    // => t_btn = sqrt(d)
    // => t_smallest = sqrt(d) + d/sqrt(d) == 2*sqrt(d)
    // SO, if t_thres < 2*sqrt(d), we can't do nothin' 🤷

    let t_smallest_sq = 4 * d;
    let t_thres_sq = t_thres * t_thres;

    if t_thres_sq < t_smallest_sq {
        return 0; // Purely to signal there ain't no way you can defeat the fastest guy here.
                  // In fact, the guy has cheated since he got a result that shouldn't be possible.
                  // I smell corruption
    }

    // Ok, so the race is clearly possible. Cool!

    let discriminant = ((t_thres_sq - 4 * d) as f64).sqrt();
    let t_btn_min = ((t_thres as f64 - discriminant) / 2f64).ceil();
    let t_btn_max = ((t_thres as f64 + discriminant) / 2f64).floor();

    (t_btn_max - t_btn_min + 1f64) as usize
}

pub struct Day6;
impl Problem for Day6 {
    const PROBLEM_DAY: u32 = 6;

    fn level1(s: &str) -> usize {
        let (min_time, fastest_dist) = s.split_once('\n').unwrap();
        let min_time = parse_stuff::<usize>(min_time.strip_prefix("Time:").unwrap());

        let fastest_dist = parse_stuff::<usize>(fastest_dist.strip_prefix("Distance:").unwrap());

        min_time
            .zip(fastest_dist)
            .map(|(t_thres, d)| margin_of_error(t_thres, d))
            .product()
    }

    fn level2(s: &str) -> usize {
        let (min_time, fastest_dist) = s.split_once('\n').unwrap();
        let min_time = min_time
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let fastest_dist = fastest_dist
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        margin_of_error(min_time, fastest_dist)
    }
}
