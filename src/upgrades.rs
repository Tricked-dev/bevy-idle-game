pub enum Multiplier {
    Speed(f32),
    Base(f64),
    Multiplier(f64),
    Addative(f64),
}
pub enum Upgrades {
    BasicSpeed(usize),
    Multiplier(usize),
    BaseUpgrade(usize),
    BasicAddative(usize),
}
impl Upgrades {
    pub fn level(&self) -> usize {
        match self {
            Self::BasicSpeed(v) => *v,
            Self::Multiplier(v) => *v,
            Self::BaseUpgrade(v) => *v,
            Self::BasicAddative(v) => *v,
        }
    }

    pub fn price(&self) -> usize {
        match self {
            Self::BasicSpeed(v) => (*v).pow(9) + *v * 3 + 200,
            Self::Multiplier(v) => (*v).pow(4) + *v * 20 + 10,
            Self::BaseUpgrade(v) => (*v).pow(5) + *v * 5 + 50,
            Self::BasicAddative(v) => (*v).pow(2) + *v * 500 + 55,
        }
    }

    pub fn display(&self) -> &'static DisplayData {
        match self {
            Self::BasicSpeed(..) => &DisplayData {
                title: "Basic speed upgrade",
                desc: "Makes money come 0.05 seconds faster",
            },
            Self::Multiplier(..) => &DisplayData {
                title: "Multiplier Upgrader!",
                desc: "Add 0.1 multiplier to your earnings!",
            },
            Self::BaseUpgrade(..) => &DisplayData {
                title: "Base Upgrade",
                desc: "Increases your base money gain by $5",
            },
            Self::BasicAddative(..) => &DisplayData {
                title: "Basic Money Upgrade",
                desc: "Adds $50 per interval",
            },
        }
    }

    pub fn stat(&self) -> Multiplier {
        match self {
            Self::BasicSpeed(v) => Multiplier::Speed(0.05 * *v as f32),
            Self::Multiplier(v) => Multiplier::Multiplier(0.1 * *v as f64),
            Self::BaseUpgrade(v) => Multiplier::Base(5.0 * *v as f64),
            Self::BasicAddative(v) => Multiplier::Addative(50.0 * *v as f64),
        }
    }

    pub fn max(&self) -> bool {
        match self {
            Self::BasicSpeed(..) => self.level() == 10,
            Self::Multiplier(..) => self.level() == 50,
            Self::BaseUpgrade(..) => self.level() == 200,
            Self::BasicAddative(..) => self.level() == 2000,
        }
    }

    // Upgrades the level and returns the new multiplier
    pub fn upgrade(&mut self) -> Multiplier {
        match self {
            Self::BasicSpeed(v) => *v += 1,
            Self::Multiplier(v) => *v += 1,
            Self::BaseUpgrade(v) => *v += 1,
            Self::BasicAddative(v) => *v += 1,
        };
        self.stat()
    }
}

pub struct DisplayData {
    pub title: &'static str,
    pub desc: &'static str,
}
