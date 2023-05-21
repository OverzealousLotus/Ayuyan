/// `Default` is implemented to conform with `tinyvec`'s requirement.
/// `Clone & Copy` is implemented for ease of use.
/// Used to determine what material equipment is made out of.
/// Where <I> is the item in particular.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Material<I> {
    #[default]
    PhantomMaterial,
    Steel(I),
    CastIron(I),
    WroughtIron(I),
    Bronze(I),
    Iron(I),
    PigIron(I),
    Brass(I),
}

/// Used to represent armour type.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Armour {
    #[default]
    PhantomArmour,
    Helmet,
    Chestplate,
    Gauntlets,
    Chausses,
    Grieves,
}

/// Used to represent weapon type.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Weapon {
    #[default]
    PhantomWeapon,
    Shortsword,
    Longsword,
    Greatsword,
    Ultrasword,
    Mace,
}
