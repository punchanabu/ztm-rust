
struct Employee<State> {
    name: String,
    state: State,
}

// blanket impl
impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state : state,
        }
    }
}

struct Agreement;
struct Signature;
struct Training;

struct FailedTraining {
    score: u8,   
}
struct OnboardingComplete {
    score: u8,
}

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Agreement,
        }
    }
    // current state is the new agreement state
    fn read_agreeement(self) -> Employee<Signature> {   
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

#[rustfmt::skip]
impl Employee<Training> {
    fn train(self, score: u8) 
        -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> 
    {
        if score >= 7 {
            Ok(self.transition(OnboardingComplete { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}
fn main() {
    let employee = Employee::new("Alice");
    let onboarded = employee.read_agreeement().sign().train(100);
    match onboarded {
        Ok(complete) => println!("onboarding complete score: {}", complete.state.score),
        Err(failed) => println!("training failed score: {}", failed.state.score)
    }
    
}