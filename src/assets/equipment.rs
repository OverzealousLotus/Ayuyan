/// `Default` is implemented to conform with `tinyvec`'s requirement.
/// `Clone & Copy` is implemented for ease of use.
/// Used to determine what material equipment is made out of.
/// Where <I> is the item in particular.
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

/// Used to represent armour type.
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

/// Used to represent weapon type.
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
