use anyhow::{bail, Result};
use glacier::{Outcome, TestResult};
use rayon::prelude::*;

fn main() -> Result<()> {
    let results = glacier::test_all()?.collect::<Result<Vec<TestResult>, _>>()?;

    let ices = results
        .iter()
        .filter(|res| matches!(res.outcome(), Outcome::ICEd));
    // consider everything that does not crash "fixed"
    let fixed = results
        .iter()
        .filter(|res| !matches!(res.outcome(), Outcome::ICEd));

    // move files to their right place:
    // ices to ices
    ices.map(|res| res.path())
        // only move ices inside "fixed" folder
        .filter(|p| p.iter().next().map(|p| p.to_str()).flatten() == Some("fixed"))
        .for_each(|p| {
            let from = p;
            let to = p.to_str().unwrap().replace("fixed", "ices");
            std::fs::rename(from, to).unwrap();
        });

    // fixed to fixed
    fixed
        .map(|res| res.path())
        // only move fixed files inside "ices" folder
        .filter(|p| p.iter().next().map(|p| p.to_str()).flatten() == Some("ices"))
        .for_each(|p| {
            let from = p;
            let to = p.to_str().unwrap().replace("ices", "fixed");
            std::fs::rename(from, to).unwrap();
        });

    Ok(())
}
