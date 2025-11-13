use rand::Rng;

// --- PLANE MODULE ---
// Contains logic specific to individual planes

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlaneStatus {
    Boarding,
    ReadyForPushback,
    TaxiingToRunway,
    ReadyForTakeoff,
    InAir,
    ReadyToLand,
    TaxiingToGate,
    AtGate,
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub id: String,
    pub status: PlaneStatus,
    /// Simple timer to simulate time-based events (e.g., boarding time)
    pub timer: i32,
}

impl Plane {
    pub fn new(id: String) -> Self {
        Plane {
            id,
            status: PlaneStatus::Boarding,
            timer: rand::thread_rng().gen_range(10..=20), // 10-20 "ticks" to board
        }
    }

    /// update is called on each "tick" of the game loop
    pub fn update(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
            return;
        }

        // When timer hits 0, advance to the next state
        match self.status {
            PlaneStatus::Boarding => {
                self.status = PlaneStatus::ReadyForPushback;
                println!("INFO: {} is fully boarded and ready for pushback.", self.id);
            }
            PlaneStatus::TaxiingToRunway => {
                self.status = PlaneStatus::ReadyForTakeoff;
                println!("INFO: {} is at the runway, ready for takeoff.", self.id);
            }
            PlaneStatus::InAir => {
                // For this demo, planes in air will eventually want to land
                self.status = PlaneStatus::ReadyToLand;
                println!("INFO: {} (in air) is requesting to land.", self.id);
            }
            PlaneStatus::TaxiingToGate => {
                self.status = PlaneStatus::AtGate;
                println!("INFO: {} has arrived at the gate.", self.id);
            }
            // Other states wait for user input
            _ => {}
        }
    }
}
