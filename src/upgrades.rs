pub enum Multiplier {
    Speed(f32),
    Base(f64),
    Multiplier(f64),
    Addative(f64),
}
pub struct DisplayData {
    pub title: &'static str,
    pub desc: &'static str,
}
/// Upgrades enum that includes all upgrades

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Upgrades {
    BasicSpeed(usize),
    Multiplier(usize),
    BaseUpgrade(usize),
    BasicAddative(usize),
    SuperBaseUpgrade(usize),
}

impl Upgrades {
    /// Returns the level of this upgrade
    pub const fn level(&self) -> usize {
        match self {
            Self::BasicSpeed(v) => *v,
            Self::Multiplier(v) => *v,
            Self::BaseUpgrade(v) => *v,
            Self::BasicAddative(v) => *v,
            Self::SuperBaseUpgrade(v) => *v,
        }
    }
    /// Returns the price of this upgrade
    pub const fn price(&self) -> usize {
        match self {
            Self::BasicSpeed(v) => (*v).pow(9) + *v * 3 + 200,
            Self::Multiplier(v) => (*v).pow(4) + *v * 20 + 10,
            Self::BaseUpgrade(v) => (*v).pow(5) + *v * 5 + 50,
            Self::BasicAddative(v) => (*v).pow(2) + *v * 500 + 55,
            Self::SuperBaseUpgrade(..) => 1000,
        }
    }
    /// Description and title from the upgrade in [`DisplayData`]
    pub const fn display(&self) -> &'static DisplayData {
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
            Self::SuperBaseUpgrade(..) => &DisplayData {
                title: "SUPER Base Upgrade",
                desc: "Upgrades base money gain by $500",
            },
        }
    }
    /// Returns the stat of this level in a [`Multiplier`]
    pub fn stat(&self) -> Multiplier {
        match self {
            Self::BasicSpeed(v) => Multiplier::Speed(0.05 * *v as f32),
            Self::Multiplier(v) => Multiplier::Multiplier(0.1 * *v as f64),
            Self::BaseUpgrade(v) => Multiplier::Base(5.0 * *v as f64),
            Self::BasicAddative(v) => Multiplier::Addative(50.0 * *v as f64),
            Self::SuperBaseUpgrade(v) => Multiplier::Base(500.0 * *v as f64),
        }
    }
    /// Returns the max level for this upgrade
    pub const fn max(&self) -> bool {
        match self {
            Self::BasicSpeed(..) => self.level() == 10,
            Self::Multiplier(..) => self.level() == 50,
            Self::BaseUpgrade(..) => self.level() == 200,
            Self::BasicAddative(..) => self.level() == 2000,
            Self::SuperBaseUpgrade(..) => self.level() == 2,
        }
    }

    /// Upgrades the level and returns the new multiplier
    pub fn upgrade(&mut self) -> Multiplier {
        match self {
            Self::BasicSpeed(v) => *v += 1,
            Self::Multiplier(v) => *v += 1,
            Self::BaseUpgrade(v) => *v += 1,
            Self::BasicAddative(v) => *v += 1,
            Self::SuperBaseUpgrade(v) => *v += 1,
        };
        self.stat()
    }
}
