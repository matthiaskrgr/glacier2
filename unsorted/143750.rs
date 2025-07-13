#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CDDAOffset {
    offset: u64,
}

impl CDDAOffset {
    const SAMPLES_PER_SECTOR: u64 = 44100 / 75;
}

impl TryFrom<u64> for CDDAOffset {
    type Error = ();

    fn try_from(offset: u64) -> Result<Self, Self::Error> {
        ((offset % Self::SAMPLES_PER_SECTOR) == 0)
            .then_some(Self { offset })
            .ok_or(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut o = CDDAOffset::default();
    let do_add = || {
        // CDDAOffset doesn't implement AddAssign
        // so this should fail to compile,
        // but the compiler panics instead
        o += CDDAOffset::try_from(0).map_err(|()| TestError)?;
        Ok(())
    };
    do_add()?;
    Ok(())
}

#[derive(Debug)]
struct TestError;

impl std::error::Error for TestError { }

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        "test error".fmt(f)
    }
}
