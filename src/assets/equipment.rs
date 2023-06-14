/// Types of materials available for weapons and armour.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Material<I> {
    Steel(I),
    CastIron(I),
    WroughtIron(I),
    Bronze(I),
    Iron(I),
    PigIron(I),
    Brass(I),
    #[default]
    PhantomMaterial,
}

/// Strength ratings for consumables.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Strength<I> {
    Potent(I),
    Nominal(I),
    Impotent(I),
    #[default]
    PhantomStrength,
}

/// Types of available Elixirs.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Elixir {
    Tenacity,
    Vitality,
    Vigor,
    Vivacity,
    #[default]
    PhantomElixir,
}

/// Types of available Armour.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Armour {
    Helmet,
    Chestplate,
    Gauntlets,
    Chausses,
    Grieves,
    #[default]
    PhantomArmour,
}

/// Types of available weaponry.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Weapon {
    Shortsword,
    Longsword,
    Greatsword,
    Ultrasword,
    Mace,
    #[default]
    PhantomWeapon,
}
