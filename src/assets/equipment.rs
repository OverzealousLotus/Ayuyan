use std::fmt;
/// Types of materials available for weapons and armour.
#[derive(Default, Clone, Copy)]
pub(crate) enum Material<I> {
    Steel(I),
    IronType(Iron<I>),
    Bronze(I),
    Brass(I),
    LeatherType(Leather<I>),
    #[default]
    PhantomMaterial,
}

impl<I: fmt::Debug> fmt::Debug for Material<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Steel(item) => write!(formatter, "Steel {:?}", item),
            Self::IronType(alloy) => write!(formatter, "{:?}", alloy),
            Self::Bronze(item) => write!(formatter, "Bronze {:?}", item),
            Self::Brass(item) => write!(formatter, "Brass {:?}", item),
            Self::LeatherType(alloy) => write!(formatter, "{:?}", alloy),
            _ => write!(formatter, "Error..."),
        }
    }
}
/// Subtypes of Iron.
#[derive(Default, Clone, Copy)]
pub(crate) enum Iron<I> {
    Cast(I),
    Wrought(I),
    Pig(I),
    #[default]
    PhantomIron,
}

impl<I: fmt::Debug> fmt::Debug for Iron<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cast(item) => write!(formatter, "Cast Iron {:?}", item),
            Self::Wrought(item) => write!(formatter, "Wrought Iron {:?}", item),
            Self::Pig(item) => write!(formatter, "Pig Iron {:?}", item),
            _ => write!(formatter, "Error..."),
        }
    }
}

/// Subtypes of Leather.
#[derive(Default, Clone, Copy)]
pub(crate) enum Leather<I> {
    Heavy(I),
    Medium(I),
    Light(I),
    #[default]
    PhantomLeather,
}

impl<I: fmt::Debug> fmt::Debug for Leather<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Heavy(item) => write!(formatter, "Heavy Leather  {:?}", item),
            Self::Medium(item) => write!(formatter, "Medium Leather {:?}", item),
            Self::Light(item) => write!(formatter, "Light Leather {:?}", item),
            _ => write!(formatter, "Error..."),
        }
    }
}

/// Strength ratings for consumables.
#[derive(Default, Clone, Copy)]
pub(crate) enum Strength<I> {
    Potent(I),
    Nominal(I),
    Impotent(I),
    Issue(I),
    #[default]
    PhantomStrength,
}

impl<I: fmt::Debug> fmt::Debug for Strength<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Potent(consumable) => write!(formatter, "Potent {:?}", consumable),
            Self::Nominal(consumable) => {
                write!(formatter, "Nominal {:?}", consumable)
            }
            Self::Impotent(consumable) => {
                write!(formatter, "Impotent {:?}", consumable)
            }
            _ => write!(formatter, "Error..."),
        }
    }
}

/// Types of available Elixirs.
#[derive(Default, Clone, Copy)]
pub(crate) enum Elixir {
    Haste,
    Vitality,
    Vigor,
    Impedance,
    Issue,
    #[default]
    PhantomElixir,
}

impl fmt::Debug for Elixir {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Haste => write!(formatter, "Elixir of Haste"),
            Self::Vitality => write!(formatter, "Elixir of Vitality"),
            Self::Vigor => write!(formatter, "Elixir of Vigor"),
            Self::Impedance => write!(formatter, "Elixir of Impedance"),
            _ => write!(formatter, "Error..."),
        }
    }
}

/// Types of available Tinctures.
#[derive(Default, Clone, Copy)]
pub(crate) enum Tincture {
    Laudanum,
    Iodyne,
    #[default]
    PhantomTincture,
}

impl fmt::Debug for Tincture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Laudanum => write!(formatter, "Ticture of Laudanum"),
            Self::Iodyne => write!(formatter, "Ticture of Iodyne"),
            _ => write!(formatter, "Error..."),
        }
    }
}

/// Types of available Armour.
#[derive(Default, Clone, Copy)]
pub(crate) enum Armour {
    Helmet,
    Chestplate,
    Gauntlets,
    Chausses,
    Grieves,
    #[default]
    PhantomArmour,
}

impl fmt::Debug for Armour {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Helmet => write!(formatter, "Helmet"),
            Self::Chestplate => write!(formatter, "Chestplate"),
            Self::Gauntlets => write!(formatter, "Gauntlets"),
            Self::Chausses => write!(formatter, "Chausses"),
            Self::Grieves => write!(formatter, "Grieves"),
            _ => write!(formatter, "Error..."),
        }
    }
}

/// Types of available weaponry.
#[derive(Default, Clone, Copy)]
pub(crate) enum Weapon {
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

impl fmt::Debug for Weapon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ultrasword => write!(formatter, "Ultrasword"),
            Self::Greatsword => write!(formatter, "Greatsword"),
            Self::Longsword => write!(formatter, "Longsword"),
            Self::Shortsword => write!(formatter, "Shortsword"),
            Self::Greatmace => write!(formatter, "Greatmace"),
            Self::Mace => write!(formatter, "Mace"),
            Self::Greatbow => write!(formatter, "Based Greatbow"),
            Self::Longbow => write!(formatter, "Based Longbow"),
            Self::Shortbow => write!(formatter, "Based Shortbow"),
            _ => write!(formatter, "Error..."),
        }
    }
}
