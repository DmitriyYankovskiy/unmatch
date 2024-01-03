mod fighter {
    enum Type {
        Any,
        Hero,
        Support,
    }
}

enum Actions {

}

struct ShemeCard {
    actions: vec<Actions>,
}


struct CombatCard {
    value: u32,
    immidietly: vec<Actions>,
    during_combat: vec<Actions>,
    after_combat: vec<Actions>,
}

enum Type {
    Combat(CombatCard),
    Sheme(Sheme),
}

struct Card {
    name: Srtring,
    class: Type,
    fighter: fighter::Type
}

fn get_deck() -> std::vec<Card> {

}