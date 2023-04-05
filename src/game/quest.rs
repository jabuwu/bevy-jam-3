use crate::UnitComposition;

#[derive(Default)]
pub struct Quest {
    pub war_chef: usize,
    pub battle: usize,
}

impl Quest {
    pub fn enemy_unit_comp(&self) -> UnitComposition {
        match self.war_chef {
            0 => match self.battle {
                0 => UnitComposition {
                    peasants: 5,
                    warriors: 0,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                1 => UnitComposition {
                    peasants: 5,
                    warriors: 2,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                2 => UnitComposition {
                    peasants: 10,
                    warriors: 1,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                _ => UnitComposition::empty(),
            },
            1 => match self.battle {
                0 => UnitComposition {
                    peasants: 2,
                    warriors: 5,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                1 => UnitComposition {
                    peasants: 5,
                    warriors: 2,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                2 => UnitComposition {
                    peasants: 10,
                    warriors: 1,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                _ => UnitComposition::empty(),
            },
            _ => UnitComposition::empty(),
        }
    }
}
