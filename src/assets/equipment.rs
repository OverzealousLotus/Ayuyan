use Armour::*;
use ArmourType::*;

// Enums are goated.
pub const ARMOUR_LOOT: [Armour; 35] = [
    Steel(Helmet),
    Steel(Chestplate),
    Steel(Gauntlets),
    Steel(Chausses),
    Steel(Grieves),
    CastIron(Helmet),
    CastIron(Chestplate),
    CastIron(Gauntlets),
    CastIron(Chausses),
    CastIron(Grieves),
    WroughtIron(Helmet),
    WroughtIron(Chestplate),
    WroughtIron(Gauntlets),
    WroughtIron(Chausses),
    WroughtIron(Grieves),
    Bronze(Helmet),
    Bronze(Chestplate),
    Bronze(Gauntlets),
    Bronze(Chausses),
    Bronze(Grieves),
    Iron(Helmet),
    Iron(Chestplate),
    Iron(Gauntlets),
    Iron(Chausses),
    Iron(Grieves),
    PigIron(Helmet),
    PigIron(Chestplate),
    PigIron(Gauntlets),
    PigIron(Chausses),
    PigIron(Grieves),
    Brass(Helmet),
    Brass(Chestplate),
    Brass(Gauntlets),
    Brass(Chausses),
    Brass(Grieves),
];

// Needs to be copyable for current use case.
// Default is implemented because of `tinyvec`'s requirement.
// Debug is debug.
#[derive(Default, Debug, Clone, Copy)]
pub enum Armour {
    // Strength is determined by strongest on top, weakest on bottom.
    #[default]
    Empty,
    Steel(ArmourType),
    CastIron(ArmourType),
    WroughtIron(ArmourType),
    Bronze(ArmourType),
    Iron(ArmourType),
    PigIron(ArmourType),
    Brass(ArmourType),
}

// Read above comments
#[derive(Default, Debug, Clone, Copy)]
pub enum ArmourType {
    #[default]
    Helmet,
    Chestplate,
    Gauntlets,
    Chausses,
    Grieves,
}
