trait Project {
    const SELF: Self;
}

fn take1(_: Project<SELF = {
    let mut res = Vec::new();
    loop {
        match decode_packet(itr) {
            Some(p) => { res.push(p); },
            None    => break
        }
    }

    return res;
}>) {}
