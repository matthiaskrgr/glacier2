struct SegmentJob {
    spec: String,
}

pub async fn run() -> Result<(), ()> {
    let jobs = get_data();
    let Some(job) = jobs.into_iter().next() else {
        println!("no jobs!");
        return Ok(())
    };

    println!("should run job {}", job.spec);

    Ok(())
}

fn get_data() -> Vec<SegmentJob> {
    Default::default()
}
