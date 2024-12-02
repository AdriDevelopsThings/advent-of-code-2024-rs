enum Monotony {
    Increasing,
    Decreasing,
}

pub fn first(input: String) -> String {
    let safe_reports = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            let mut monotony: Option<Monotony> = None;
            let mut last_num: Option<u32> = None;
            for r in report {
                if let Some(ln) = last_num {
                    if ln.abs_diff(*r) == 0 || ln.abs_diff(*r) > 3 {
                        return false;
                    }

                    if let Some(m) = &monotony {
                        if *r > ln && !matches!(m, Monotony::Increasing) {
                            return false;
                        }
                        if *r < ln && !matches!(m, Monotony::Decreasing) {
                            return false;
                        }
                    } else if *r > ln {
                        monotony = Some(Monotony::Increasing)
                    } else {
                        monotony = Some(Monotony::Decreasing)
                    }
                }
                last_num = Some(*r)
            }
            true
        })
        .collect::<Vec<_>>()
        .len();

    safe_reports.to_string()
}

pub fn second(input: String) -> String {
    let safe_reports = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            let mut monotony: Option<Monotony> = None;
            let mut last_num: Option<u32> = None;
            let mut skipped = false;
            for r in report {
                if let Some(ln) = last_num {
                    if ln.abs_diff(*r) == 0 || ln.abs_diff(*r) > 3 {
                        if skipped {
                            return false;
                        }
                        skipped = true;
                    }

                    if let Some(m) = &monotony {
                        if *r > ln && !matches!(m, Monotony::Increasing) {
                            if skipped {
                                return false;
                            }
                            skipped = true;
                        }
                        if *r < ln && !matches!(m, Monotony::Decreasing) {
                            if skipped {
                                return false;
                            }
                            skipped = true;
                        }
                    } else if *r > ln {
                        monotony = Some(Monotony::Increasing)
                    } else {
                        monotony = Some(Monotony::Decreasing)
                    }
                }
                last_num = Some(*r)
            }
            true
        })
        .collect::<Vec<_>>()
        .len();

    safe_reports.to_string()
}
