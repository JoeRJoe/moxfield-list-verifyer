#[derive(Debug, Clone, Default)]
pub struct ValidationResults {
    pub mass_land_denial_cards: Vec<(String, String)>,
    pub non_land_tutors: Vec<(String, String)>,
    pub commander_tutors: Vec<String>,
    pub two_card_combos: Vec<(Vec<String>, String)>,
    pub gamechangers: Vec<String>,
    pub infinite_turns_combos: Vec<Vec<String>>,
    pub combos: Vec<(Vec<String>, String)>,
}

impl ValidationResults {
    pub fn merge(mut self, other: ValidationResults) -> Self {
        self.mass_land_denial_cards
            .extend(other.mass_land_denial_cards);
        self.non_land_tutors.extend(other.non_land_tutors);
        self.commander_tutors.extend(other.commander_tutors);
        self.two_card_combos.extend(other.two_card_combos);
        self.gamechangers.extend(other.gamechangers);
        self.infinite_turns_combos
            .extend(other.infinite_turns_combos);

        if self.combos.is_empty() && !other.combos.is_empty() {
            self.combos = other.combos;
        }

        self
    }

    pub fn is_valid(&self) -> bool {
        self.mass_land_denial_cards.is_empty()
            && self.non_land_tutors.len() <= 3
            && self.commander_tutors.is_empty()
            && self.two_card_combos.is_empty()
            && self.gamechangers.is_empty()
            && self.infinite_turns_combos.is_empty()
    }
}
