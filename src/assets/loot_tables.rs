use crate::assets::equipment::{Armour, Elixir, Material, Strength, Tincture, Weapon};
use crate::assets::equipment::{
    Armour::*, Iron::*, Leather::*, Material::*, Strength::*, Weapon::*,
};

/// Array used to store armours.
/// Strength goes from highest being strongest, to lowest being weakest.
pub(crate) const ARMOUR_LOOT: [Material<Armour>; 45] = [
    // Steel
    Steel(Helmet),
    Steel(Chestplate),
    Steel(Gauntlets),
    Steel(Chausses),
    Steel(Grieves),
    // Cast Iron
    IronType(Cast(Helmet)),
    IronType(Cast(Chestplate)),
    IronType(Cast(Gauntlets)),
    IronType(Cast(Chausses)),
    IronType(Cast(Grieves)),
    // Wrought Iron
    IronType(Wrought(Helmet)),
    IronType(Wrought(Chestplate)),
    IronType(Wrought(Gauntlets)),
    IronType(Wrought(Chausses)),
    IronType(Wrought(Grieves)),
    // Bronze
    Bronze(Helmet),
    Bronze(Chestplate),
    Bronze(Gauntlets),
    Bronze(Chausses),
    Bronze(Grieves),
    // Pig Iron
    IronType(Pig(Helmet)),
    IronType(Pig(Chestplate)),
    IronType(Pig(Gauntlets)),
    IronType(Pig(Chausses)),
    IronType(Pig(Grieves)),
    // Heavy Leather
    LeatherType(Heavy(Helmet)),
    LeatherType(Heavy(Chestplate)),
    LeatherType(Heavy(Gauntlets)),
    LeatherType(Heavy(Chausses)),
    LeatherType(Heavy(Grieves)),
    // Brass
    Brass(Helmet),
    Brass(Chestplate),
    Brass(Gauntlets),
    Brass(Chausses),
    Brass(Grieves),
    // Medium leather
    LeatherType(Medium(Helmet)),
    LeatherType(Medium(Chestplate)),
    LeatherType(Medium(Gauntlets)),
    LeatherType(Medium(Chausses)),
    LeatherType(Medium(Grieves)),
    // Light Leather
    LeatherType(Light(Helmet)),
    LeatherType(Light(Chestplate)),
    LeatherType(Light(Gauntlets)),
    LeatherType(Light(Chausses)),
    LeatherType(Light(Grieves)),
];

/// Array used to store weaponry.
pub(crate) const WEAPON_LOOT: [Material<Weapon>; 54] = [
    // Steel Weaponry
    Steel(Ultrasword),
    Steel(Greatsword),
    Steel(Longsword),
    Steel(Shortsword),
    Steel(Greatmace),
    Steel(Mace),
    Steel(Greatbow),
    Steel(Longbow),
    Steel(Shortbow),
    // Cast Iron Weaponry
    IronType(Cast(Ultrasword)),
    IronType(Cast(Greatsword)),
    IronType(Cast(Longsword)),
    IronType(Cast(Shortsword)),
    IronType(Cast(Greatmace)),
    IronType(Cast(Mace)),
    IronType(Cast(Greatbow)),
    IronType(Cast(Longbow)),
    IronType(Cast(Shortbow)),
    // Wrought Iron Weaponry
    IronType(Wrought(Ultrasword)),
    IronType(Wrought(Greatsword)),
    IronType(Wrought(Longsword)),
    IronType(Wrought(Shortsword)),
    IronType(Wrought(Greatmace)),
    IronType(Wrought(Mace)),
    IronType(Wrought(Greatbow)),
    IronType(Wrought(Longbow)),
    IronType(Wrought(Shortbow)),
    // Bronze Weaponry
    Bronze(Ultrasword),
    Bronze(Greatsword),
    Bronze(Longsword),
    Bronze(Shortsword),
    Bronze(Greatmace),
    Bronze(Mace),
    Bronze(Greatbow),
    Bronze(Longbow),
    Bronze(Shortbow),
    // Pig Iron Weaponry
    IronType(Pig(Ultrasword)),
    IronType(Pig(Greatsword)),
    IronType(Pig(Longsword)),
    IronType(Pig(Shortsword)),
    IronType(Pig(Greatmace)),
    IronType(Pig(Mace)),
    IronType(Pig(Greatbow)),
    IronType(Pig(Longbow)),
    IronType(Pig(Shortbow)),
    // Brass Weaponry
    Brass(Ultrasword),
    Brass(Greatsword),
    Brass(Longsword),
    Brass(Shortsword),
    Brass(Greatmace),
    Brass(Mace),
    Brass(Greatbow),
    Brass(Longbow),
    Brass(Shortbow),
];

/// Array storing Elixirs.
pub(crate) const ELIXIR_LOOT: [Strength<Elixir>; 12] = [
    // Haste
    Potent(Elixir::Haste),
    Nominal(Elixir::Haste),
    Impotent(Elixir::Haste),
    // Vigor
    Potent(Elixir::Vigor),
    Nominal(Elixir::Vigor),
    Impotent(Elixir::Vigor),
    // Vitality
    Potent(Elixir::Vitality),
    Nominal(Elixir::Vitality),
    Impotent(Elixir::Vitality),
    // Impedance
    Potent(Elixir::Impedance),
    Nominal(Elixir::Impedance),
    Impotent(Elixir::Impedance),
];

pub(crate) const TINCTURE_LOOT: [Strength<Tincture>; 6] = [
    // Laudanum
    Potent(Tincture::Laudanum),
    Nominal(Tincture::Laudanum),
    Impotent(Tincture::Laudanum),
    // Iodyne
    Potent(Tincture::Iodyne),
    Nominal(Tincture::Iodyne),
    Impotent(Tincture::Iodyne),
];
