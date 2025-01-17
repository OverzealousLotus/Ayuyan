use crate::assets::equipment::{Armour, Elixir, Tincture, Weapon};
use crate::assets::equipment::{
    ArmourType::{Helmet, Chestplate, Gauntlets, Chausses, Grieves},
    ElixirType::{Haste, Vigor, Impedance, Vitality},
    Iron::{Cast, Pig, Wrought},
    Leather::{Light, Medium, Heavy},
    Material::{Stone, Brass, Bronze, LeatherType, IronType, SteelType},
    Potency::{Impotent, Diluted, Nominal, Potent},
    Steel::{Common, Ebon, Ivory},
    TinctureType::{Laudanum, Iodyne},
    WeaponType::{Greatbow, Greatmace, Greatsword, Longbow, Longsword, Mace, Shortbow, Shortsword, Ultrasword},
};

/// Array used to store armours.
/// Strength goes from highest being strongest, to lowest being weakest.
pub(crate) const ARMOUR_LOOT: [Armour; 55] = [
    // Ivory Steel
    Armour::new(Helmet, SteelType(Ivory)),
    Armour::new(Chestplate, SteelType(Ivory)),
    Armour::new(Gauntlets, SteelType(Ivory)),
    Armour::new(Chausses, SteelType(Ivory)),
    Armour::new(Grieves, SteelType(Ivory)),
    // Ebonsteel
    Armour::new(Helmet, SteelType(Ebon)),
    Armour::new(Chestplate, SteelType(Ebon)),
    Armour::new(Gauntlets, SteelType(Ebon)),
    Armour::new(Chausses, SteelType(Ebon)),
    Armour::new(Grieves, SteelType(Ebon)),
    // Common Steel
    Armour::new(Helmet, SteelType(Common)),
    Armour::new(Chestplate, SteelType(Common)),
    Armour::new(Gauntlets, SteelType(Common)),
    Armour::new(Chausses, SteelType(Common)),
    Armour::new(Grieves, SteelType(Common)),
    // Cast Iron
    Armour::new(Helmet, IronType(Cast)),
    Armour::new(Chestplate, IronType(Cast)),
    Armour::new(Gauntlets, IronType(Cast)),
    Armour::new(Chausses, IronType(Cast)),
    Armour::new(Grieves, IronType(Cast)),
    // Wrought Iron
    Armour::new(Helmet, IronType(Wrought)),
    Armour::new(Chestplate, IronType(Wrought)),
    Armour::new(Gauntlets, IronType(Wrought)),
    Armour::new(Chausses, IronType(Wrought)),
    Armour::new(Grieves, IronType(Wrought)),
    // Bronze
    Armour::new(Helmet, Bronze),
    Armour::new(Chestplate, Bronze),
    Armour::new(Gauntlets, Bronze),
    Armour::new(Chausses, Bronze),
    Armour::new(Grieves, Bronze),
    // Pig Iron
    Armour::new(Helmet, IronType(Pig)),
    Armour::new(Chestplate, IronType(Pig)),
    Armour::new(Gauntlets, IronType(Pig)),
    Armour::new(Chausses, IronType(Pig)),
    Armour::new(Grieves, IronType(Pig)),
    // Heavy Leather
    Armour::new(Helmet, LeatherType(Heavy)),
    Armour::new(Chestplate, LeatherType(Heavy)),
    Armour::new(Gauntlets, LeatherType(Heavy)),
    Armour::new(Chausses, LeatherType(Heavy)),
    Armour::new(Grieves, LeatherType(Heavy)),
    // Brass
    Armour::new(Helmet, Brass),
    Armour::new(Chestplate, Brass),
    Armour::new(Gauntlets, Brass),
    Armour::new(Chausses, Brass),
    Armour::new(Grieves, Brass),
    // Medium leather
    Armour::new(Helmet, LeatherType(Medium)),
    Armour::new(Chestplate, LeatherType(Medium)),
    Armour::new(Gauntlets, LeatherType(Medium)),
    Armour::new(Chausses, LeatherType(Medium)),
    Armour::new(Grieves, LeatherType(Medium)),
    // Light Leather
    Armour::new(Helmet, LeatherType(Light)),
    Armour::new(Chestplate, LeatherType(Light)),
    Armour::new(Gauntlets, LeatherType(Light)),
    Armour::new(Chausses, LeatherType(Light)),
    Armour::new(Grieves, LeatherType(Light)),
];

/// Array used to store weaponry.
pub(crate) const WEAPON_LOOT: [Weapon; 60] = [
    // Ivory Steel Weaponry
    Weapon::new(Ultrasword, SteelType(Ivory)),
    Weapon::new(Greatsword, SteelType(Ivory)),
    Weapon::new(Longsword, SteelType(Ivory)),
    Weapon::new(Shortsword, SteelType(Ivory)),
    Weapon::new(Greatmace, SteelType(Ivory)),
    Weapon::new(Mace, SteelType(Ivory)),
    Weapon::new(Greatbow, SteelType(Ivory)),
    Weapon::new(Longbow, SteelType(Ivory)),
    Weapon::new(Shortbow, SteelType(Ivory)),
    // Cast Iron Weaponry
    Weapon::new(Ultrasword, IronType(Cast)),
    Weapon::new(Greatsword, IronType(Cast)),
    Weapon::new(Longsword, IronType(Cast)),
    Weapon::new(Shortsword, IronType(Cast)),
    Weapon::new(Greatmace, IronType(Cast)),
    Weapon::new(Mace, IronType(Cast)),
    Weapon::new(Greatbow, IronType(Cast)),
    Weapon::new(Longbow, IronType(Cast)),
    Weapon::new(Shortbow, IronType(Cast)),
    // Wrought Iron Weaponry
    Weapon::new(Ultrasword, IronType(Wrought)),
    Weapon::new(Greatsword, IronType(Wrought)),
    Weapon::new(Longsword, IronType(Wrought)),
    Weapon::new(Shortsword, IronType(Wrought)),
    Weapon::new(Greatmace, IronType(Wrought)),
    Weapon::new(Mace, IronType(Wrought)),
    Weapon::new(Greatbow, IronType(Wrought)),
    Weapon::new(Longbow, IronType(Wrought)),
    Weapon::new(Shortbow, IronType(Wrought)),
    // Bronze Weaponry
    Weapon::new(Ultrasword, Bronze),
    Weapon::new(Greatsword, Bronze),
    Weapon::new(Longsword, Bronze),
    Weapon::new(Shortsword, Bronze),
    Weapon::new(Greatmace, Bronze),
    Weapon::new(Mace, Bronze),
    Weapon::new(Greatbow, Bronze),
    Weapon::new(Longbow, Bronze),
    Weapon::new(Shortbow, Bronze),
    // Pig Iron Weaponry
    Weapon::new(Ultrasword, IronType(Pig)),
    Weapon::new(Greatsword, IronType(Pig)),
    Weapon::new(Longsword, IronType(Pig)),
    Weapon::new(Shortsword, IronType(Pig)),
    Weapon::new(Greatmace, IronType(Pig)),
    Weapon::new(Mace, IronType(Pig)),
    Weapon::new(Greatbow, IronType(Pig)),
    Weapon::new(Longbow, IronType(Pig)),
    Weapon::new(Shortbow, IronType(Pig)),
    // Brass Weaponry
    Weapon::new(Ultrasword, Brass),
    Weapon::new(Greatsword, Brass),
    Weapon::new(Longsword, Brass),
    Weapon::new(Shortsword, Brass),
    Weapon::new(Greatmace, Brass),
    Weapon::new(Mace, Brass),
    Weapon::new(Greatbow, Brass),
    Weapon::new(Longbow, Brass),
    Weapon::new(Shortbow, Brass),
    // Stone Weaponry
    Weapon::new(Ultrasword, Stone),
    Weapon::new(Greatsword, Stone),
    Weapon::new(Longsword, Stone),
    Weapon::new(Shortsword, Stone),
    Weapon::new(Greatmace, SteelType(Ivory)),
    Weapon::new(Mace, Stone),
];

/// Array storing Elixirs.
pub(crate) const ELIXIR_LOOT: [Elixir; 16] = [
    // Haste
    Elixir::new(Haste, Potent),
    Elixir::new(Haste, Nominal),
    Elixir::new(Haste, Impotent),
    Elixir::new(Haste, Diluted),
    // Vigor
    Elixir::new(Vigor, Potent),
    Elixir::new(Vigor, Nominal),
    Elixir::new(Vigor, Impotent),
    Elixir::new(Vigor, Diluted),
    // Vitality
    Elixir::new(Vitality, Potent),
    Elixir::new(Vitality, Nominal),
    Elixir::new(Vitality, Impotent),
    Elixir::new(Vitality, Diluted),
    // Impedance
    Elixir::new(Impedance, Potent),
    Elixir::new(Impedance, Nominal),
    Elixir::new(Impedance, Impotent),
    Elixir::new(Impedance, Diluted),
];

pub(crate) const TINCTURE_LOOT: [Tincture; 8] = [
    // Laudanum
    Tincture::new(Laudanum, Potent),
    Tincture::new(Laudanum, Nominal),
    Tincture::new(Laudanum, Impotent),
    Tincture::new(Laudanum, Diluted),
    // Iodyne
    Tincture::new(Iodyne, Potent),
    Tincture::new(Iodyne, Nominal),
    Tincture::new(Iodyne, Impotent),
    Tincture::new(Iodyne, Diluted),
];
