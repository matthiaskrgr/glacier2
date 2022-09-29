use anyhow::Result;
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
        .map(|path| {
            // the path of an ice file
            let from = path;
            let mut to = std::path::PathBuf::from("ices");
            to.push(from.file_name().unwrap());
            (from, to)
        })
        // don't move if src == dest
        .filter(|(from, to)| from != to)
        .for_each(|(from, to)| {
            std::fs::rename(from, to).unwrap();
        });

    // fixed to fixed
    fixed
        .map(|res| res.path())
        .map(|path| {
            // the path of file that does not ice file
            let from = path;
            let mut to = std::path::PathBuf::from("fixed");
            to.push(from.file_name().unwrap());
            (from, to)
        })
        // don't move if src == dest
        .filter(|(from, to)| from != to)
        .for_each(|(from, to)| {
            std::fs::rename(from, to).unwrap();
        });

    Ok(())
}
