use std::fmt;

/// Struct representing a Weapon item.
#[derive(Default, Clone, Copy)]
pub(crate) struct Weapon {
    variant: WeaponType,
    material: Material,
}

impl fmt::Display for Weapon {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} {}", self.material, self.variant)
    }
}

impl Weapon {
    pub(crate) const fn new(variant: WeaponType, material: Material) -> Self {
        Weapon { variant, material }
    }
}

/// Struct representing an Armour item.
#[derive(Default, Clone, Copy)]
pub(crate) struct Armour {
    variant: ArmourType,
    material: Material,
}

impl fmt::Display for Armour {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} {}", self.material, self.variant)
    }
}

impl Armour {
    pub(crate) const fn new(variant: ArmourType, material: Material) -> Self {
        Armour { variant, material }
    }
}

/// Types of available materials for weapons and armour.
#[derive(Default, Clone, Copy)]
pub(crate) enum Material {
    SteelType(Steel),
    IronType(Iron),
    Bronze,
    Brass,
    Stone,
    LeatherType(Leather),
    #[default]
    PhantomVariant,
}

impl fmt::Display for Material {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SteelType(steel) => write!(formatter, "{steel}"),
            Self::IronType(iron) => write!(formatter, "{iron}"),
            Self::Bronze => write!(formatter, "Bronze"),
            Self::Brass => write!(formatter, "Brass"),
            Self::Stone => write!(formatter, "Stone"),
            Self::LeatherType(leather) => write!(formatter, "{leather}"),
            Self::PhantomVariant => write!(formatter, "Error..."),
        }
    }
}

/// Subtypes of Steel.
#[derive(Default, Clone, Copy)]
pub(crate) enum Steel {
    Ivory,
    Ebon,
    Common,
    #[default]
    PhantomSteelVar,
}

impl fmt::Display for Steel {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ivory => write!(formatter, "Ivory Steel"),
            Self::Ebon => write!(formatter, "Ebonsteel"),
            Self::Common => write!(formatter, "Common Steel"),
            Self::PhantomSteelVar => write!(formatter, "Error..."),
        }
    }
}

/// Subtypes of Iron.
#[derive(Default, Clone, Copy)]
pub(crate) enum Iron {
    Cast,
    Wrought,
    Pig,
    #[default]
    PhantomIronVar,
}

impl fmt::Display for Iron {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Cast => write!(formatter, "Cast Iron"),
            Self::Wrought => write!(formatter, "Wrought Iron"),
            Self::Pig => write!(formatter, "Pig Iron"),
            Self::PhantomIronVar => write!(formatter, "Error..."),
        }
    }
}

/// Subtypes of Leather.
#[derive(Default, Clone, Copy)]
pub(crate) enum Leather {
    Heavy,
    Medium,
    Light,
    #[default]
    PhantomLeatherVar,
}

impl fmt::Display for Leather {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Heavy => write!(formatter, "Heavy Leather"),
            Self::Medium => write!(formatter, "Medium Leather"),
            Self::Light => write!(formatter, "Light Leather"),
            Self::PhantomLeatherVar => write!(formatter, "Error..."),
        }
    }
}

/// Struct representing an Elixir item.
#[derive(Default, Clone, Copy)]
pub(crate) struct Elixir {
    variant: ElixirType,
    potency: Potency,
}

impl fmt::Display for Elixir {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} {}", self.potency, self.variant)
    }
}

impl Elixir {
    pub(crate) const fn new(variant: ElixirType, potency: Potency) -> Self {
        Elixir { variant, potency }
    }
}

/// Struct representing a Tincture item.
#[derive(Default, Clone, Copy)]
pub(crate) struct Tincture {
    variant: TinctureType,
    potency: Potency,
}

impl fmt::Display for Tincture {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} {}", self.potency, self.variant)
    }
}

impl Tincture {
    pub(crate) const fn new(variant: TinctureType, potency: Potency) -> Self {
        Tincture { variant, potency }
    }
}

/// Potency ratings for consumables.
#[derive(Default, Clone, Copy)]
pub(crate) enum Potency {
    Potent,
    Nominal,
    Impotent,
    Diluted,
    #[default]
    PhantomPotencyVar,
}

impl fmt::Display for Potency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Potent => write!(formatter, "Potent"),
            Self::Nominal => write!(formatter, "Nominal"),
            Self::Impotent => write!(formatter, "Impotent"),
            Self::Diluted => write!(formatter, "Diluted"),
            Self::PhantomPotencyVar => write!(formatter, "Error..."),
        }
    }
}

/// Elixirs are Potions for me. If this confuses you, feel free to change it, as they refer-
/// -to the same concept. The variant names can also be swapped out.
#[derive(Default, Clone, Copy)]
pub(crate) enum ElixirType {
    Haste,
    Vitality,
    Vigor,
    Impedance,
    #[default]
    PhantomElixir,
}

impl fmt::Display for ElixirType {
    /// Make sure to modify this if you change variant names, or add new ones.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Haste => write!(formatter, "Elixir of Haste"),
            Self::Vitality => write!(formatter, "Elixir of Vitality"),
            Self::Vigor => write!(formatter, "Elixir of Vigor"),
            Self::Impedance => write!(formatter, "Elixir of Impedance"),
            Self::PhantomElixir => write!(formatter, "Error..."),
        }
    }
}

/// Tinctures are medicinal brews, rather than "potions" in my use-case.
/// This can be removed if you have no interest, or merge it into `Elixir`
#[derive(Default, Clone, Copy)]
pub(crate) enum TinctureType {
    Laudanum,
    Iodyne,
    #[default]
    PhantomTincture,
}

impl fmt::Display for TinctureType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Laudanum => write!(formatter, "Ticture of Laudanum"),
            Self::Iodyne => write!(formatter, "Ticture of Iodyne"),
            Self::PhantomTincture => write!(formatter, "Error..."),
        }
    }
}

/// Armours are simplified for my use-case.
/// However, this can easily be extended to suit others' needs.
#[derive(Default, Clone, Copy)]
pub(crate) enum ArmourType {
    Helmet,
    Chestplate,
    Gauntlets,
    Chausses,
    Grieves,
    #[default]
    PhantomArmour,
}

impl fmt::Display for ArmourType {
    /// To extend, just add another match arm for your variant.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Helmet => write!(formatter, "Helmet"),
            Self::Chestplate => write!(formatter, "Chestplate"),
            Self::Gauntlets => write!(formatter, "Gauntlets"),
            Self::Chausses => write!(formatter, "Chausses"),
            Self::Grieves => write!(formatter, "Grieves"),
            Self::PhantomArmour => write!(formatter, "Error..."),
        }
    }
}

/// I expect this enum to vastly expand.
/// There are so many weapons, having a simple few isn't enough.
#[derive(Default, Clone, Copy)]
pub(crate) enum WeaponType {
    Ultrasword,
    Greatsword,
    Longsword,
    Shortsword,
    Greatmace,
    Mace,
    Greatbow,
    Longbow,
    Shortbow,
    #[default]
    PhantomWeapon,
}

impl fmt::Display for WeaponType {
    /// Weaponry is easily concated into being: "Bronze Longsword", but
    /// bows themselves are usually made of a flexible material.
    /// So, having them made entirely out of iron makes no sense.
    /// Therefore, they are instead: "Bronze Lined Longbow"
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ultrasword => write!(formatter, "Ultrasword"),
            Self::Greatsword => write!(formatter, "Greatsword"),
            Self::Longsword => write!(formatter, "Longsword"),
            Self::Shortsword => write!(formatter, "Shortsword"),
            Self::Greatmace => write!(formatter, "Greatmace"),
            Self::Mace => write!(formatter, "Mace"),
            Self::Greatbow => write!(formatter, "Lined Greatbow"),
            Self::Longbow => write!(formatter, "Lined Longbow"),
            Self::Shortbow => write!(formatter, "Lined Shortbow"),
            Self::PhantomWeapon => write!(formatter, "Error..."),
        }
    }
}
