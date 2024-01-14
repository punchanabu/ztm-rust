



#[derive(Copy, Clone, Debug)]
struct LuggageId(usize);
#[derive(Debug)]
struct Luggage(LuggageId);

struct CheckIn(LuggageId);

struct OnLog(LuggageId);

struct OffLoad(LuggageId);

struct AwaitingPickUp(LuggageId);

struct EndCustody(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Luggage(id)
    }
    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn on_log(self) -> OnLog {
        OnLog(self.0)
    }
}

impl OnLog {
    fn off_load(self) -> OffLoad {
        OffLoad(self.0)
    }
}

impl OffLoad {
    fn awaiting_pick_up(self) -> AwaitingPickUp {
        AwaitingPickUp(self.0)
    }
}

impl AwaitingPickUp {
    fn pick_up(self) -> (Luggage,EndCustody) {
        (Luggage(self.0),EndCustody(self.0))
    }
}


fn main() {
    let luggage = Luggage::new(LuggageId(334940349303));
    let luggage = luggage.check_in().on_log().off_load().awaiting_pick_up();
    let (luggage,_) = luggage.pick_up();
    
    println!("Luggage: {:?}", luggage);
}