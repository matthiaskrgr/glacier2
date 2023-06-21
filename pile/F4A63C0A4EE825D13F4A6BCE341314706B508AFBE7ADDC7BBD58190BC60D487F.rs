const bad : u32 = {
    {
        5;
        0
    }
};

const bad_two : u32 = {
    {
        invalid();
        //~^ ERROR: cannot call non-const fn `invalid`
        0
    }
};

const bad_three : u32 = {
    {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
}
};

static bad_four : u32 = {
    {
        5;
        0
    }
};

static bad_five : u32 = {
    {
        invalid();
        //~^ ERROR: cannot call non-const fn `invalid`
        0
    }
};

static bad_six : u32 = {
    {
        valid();
        0
    }
};

static mut bad_seven : u32 = {
    {
        5;
        0
    }
};

static mut bad_eight : u32 = {
    {
        invalid();
        //~^ ERROR: cannot call non-const fn `invalid`
        0
    }
};

static mut bad_nine : u32 = {
    {
        valid();
        0
    }
};


fn invalid() {}
const fn valid() {}

fn main() {}
