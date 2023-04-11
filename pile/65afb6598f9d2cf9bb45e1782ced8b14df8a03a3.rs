trait Traitor<'a, const M: Traitor = Traitor> {
    fn owo<const Traitor: Traitor = Traitor, const M: Traitor = Traitor>(&self) -> Traitor {
        Traitor
    }
}

trait Traitor<const Traitor: Traitor = Traitor, const M: owo = {0}> {
    fn owo<const Traitor:  Traitor= Traitor, const M: Traitor = Traitor>(&self) -> o(&self) -> Traitor {
        Traitor
    }
}
