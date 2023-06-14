use tinyrand::{Rand, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use crate::assets::equipment::{Armour, Elixir, Iron, Material, Strength, Weapon};
use crate::Context;

// Function to create a seed, then return it.
pub(crate) async fn gen_num(limit: usize) -> usize {
    let mut seed = ClockSeed::default();

    let mut random_num = StdRand::seed(seed.next_u64());

    random_num.next_lim_usize(limit)
}

// Simple function to simplify, and handle message sending.
pub(crate) async fn speak(context: Context<'_>, message: &str) {
    if let Err(reason) = context.say(message).await {
        println!(
            "An error occurred while trying to send message: {}, with reason: {}",
            message, reason
        );
    } else {
        println!("Message: {} | Successfully sent!", message);
    }
}

pub(crate) async fn get_elixir(elixir: Strength<Elixir>) -> &'static str {
    match elixir {
        Strength::Potent(Elixir::Tenacity) => "Potent Elixir of Tenacity",
        Strength::Potent(Elixir::Vigor) => "Potent Elixir of Vigor",
        Strength::Potent(Elixir::Vitality) => "Potent Elixir of Vitality",
        Strength::Potent(Elixir::Vivacity) => "Potent Elixir of Vivacity",

        Strength::Nominal(Elixir::Tenacity) => "Nominal Elixir of Tenacity",
        Strength::Nominal(Elixir::Vigor) => "Nominal Elixir of Vigor",
        Strength::Nominal(Elixir::Vitality) => "Nominal Elixir of Vitality",
        Strength::Nominal(Elixir::Vivacity) => "Nominal Elixir of Vivacity",

        Strength::Impotent(Elixir::Tenacity) => "Impotent Elixir of Tenacity",
        Strength::Impotent(Elixir::Vigor) => "Impotent Elixir of Vigor",
        Strength::Impotent(Elixir::Vitality) => "Impotent Elixir of Vitality",
        Strength::Impotent(Elixir::Vivacity) => "Impotent Elixir of Vivacity",
        _ => "Error",
    }
}

pub(crate) async fn get_weapon(weapon: Material<Weapon>) -> &'static str {
    match weapon {
        Material::Steel(Weapon::Ultrasword) => "Steel Ultrasword",
        Material::Steel(Weapon::Greatsword) => "Steel Greatsword",
        Material::Steel(Weapon::Longsword) => "Steel Longsword",
        Material::Steel(Weapon::Shortsword) => "Steel Shortsword",
        Material::Steel(Weapon::Mace) => "Steel Mace",

        Material::IronType(Iron::Cast(Weapon::Ultrasword)) => "Cast Iron Ultrasword",
        Material::IronType(Iron::Cast(Weapon::Greatsword)) => "Cast Iron Greatsword",
        Material::IronType(Iron::Cast(Weapon::Longsword)) => "Cast Iron Longsword",
        Material::IronType(Iron::Cast(Weapon::Shortsword)) => "Cast Iron Longsword",
        Material::IronType(Iron::Cast(Weapon::Mace)) => "Cast Iron Mace",

        Material::Bronze(Weapon::Ultrasword) => "Bronze Ultrasword",
        Material::Bronze(Weapon::Greatsword) => "Bronze Greatsword",
        Material::Bronze(Weapon::Longsword) => "Bronze Longsword",
        Material::Bronze(Weapon::Shortsword) => "Bronze Shortsword",
        Material::Bronze(Weapon::Mace) => "Bronze Mace",

        Material::IronType(Iron::Wrought(Weapon::Ultrasword)) => "Wrought Iron Ultrasword",
        Material::IronType(Iron::Wrought(Weapon::Greatsword)) => "Wrought Iron Greatsword",
        Material::IronType(Iron::Wrought(Weapon::Longsword)) => "Wrought Iron Longsword",
        Material::IronType(Iron::Wrought(Weapon::Shortsword)) => "Wrought Iron Shortsword",
        Material::IronType(Iron::Wrought(Weapon::Mace)) => "Wrought Iron Mace",

        Material::IronType(Iron::Pig(Weapon::Ultrasword)) => "Pig Iron Ultrasword",
        Material::IronType(Iron::Pig(Weapon::Greatsword)) => "Pig Iron Greatsword",
        Material::IronType(Iron::Pig(Weapon::Longsword)) => "Pig Iron Longsword",
        Material::IronType(Iron::Pig(Weapon::Shortsword)) => "Pig Iron Shortsword",
        Material::IronType(Iron::Pig(Weapon::Mace)) => "Pig Iron Mace",

        Material::Brass(Weapon::Ultrasword) => "Brass Ultrasword",
        Material::Brass(Weapon::Greatsword) => "Brass Greatsword",
        Material::Brass(Weapon::Longsword) => "Brass Longsword",
        Material::Brass(Weapon::Shortsword) => "Brass Shortsword",
        Material::Brass(Weapon::Mace) => "Brass Mace",

        _ => "Error",
    }
}

pub(crate) async fn get_armour(armour: Material<Armour>) -> &'static str {
    match armour {
        Material::Steel(Armour::Helmet) => "Steel Helmet",
        Material::Steel(Armour::Chestplate) => "Steel Chestplate",
        Material::Steel(Armour::Gauntlets) => "Steel Gauntlets",
        Material::Steel(Armour::Chausses) => "Steel Chausses",
        Material::Steel(Armour::Grieves) => "Steel Grieves",

        Material::IronType(Iron::Cast(Armour::Helmet)) => "Cast Iron Helmet",
        Material::IronType(Iron::Cast(Armour::Chestplate)) => "Cast Iron Chestplate",
        Material::IronType(Iron::Cast(Armour::Gauntlets)) => "Cast Iron Gauntlets",
        Material::IronType(Iron::Cast(Armour::Chausses)) => "Cast Iron Chausses",
        Material::IronType(Iron::Cast(Armour::Grieves)) => "Cast Iron Grieves",

        Material::Bronze(Armour::Helmet) => "Bronze Helmet",
        Material::Bronze(Armour::Chestplate) => "Bronze Chestplate",
        Material::Bronze(Armour::Gauntlets) => "Bronze Gauntlets",
        Material::Bronze(Armour::Chausses) => "Bronze Chausses",
        Material::Bronze(Armour::Grieves) => "Bronze Grieves",

        Material::IronType(Iron::Wrought(Armour::Helmet)) => "Wrought Iron Helmet",
        Material::IronType(Iron::Wrought(Armour::Chestplate)) => "Wrought Iron Chestplate",
        Material::IronType(Iron::Wrought(Armour::Gauntlets)) => "Wrought Iron Gauntlets",
        Material::IronType(Iron::Wrought(Armour::Chausses)) => "Wrought Iron Chausses",
        Material::IronType(Iron::Wrought(Armour::Grieves)) => "Wrought Iron Grieves",

        Material::IronType(Iron::Pig(Armour::Helmet)) => "Pig Iron Helmet",
        Material::IronType(Iron::Pig(Armour::Chestplate)) => "Pig Iron Chestplate",
        Material::IronType(Iron::Pig(Armour::Gauntlets)) => "Pig Iron Gauntlets",
        Material::IronType(Iron::Pig(Armour::Chausses)) => "Pig Iron Chausses",
        Material::IronType(Iron::Pig(Armour::Grieves)) => "Pig Iron Grieves",

        Material::Brass(Armour::Helmet) => "Brass Helmet",
        Material::Brass(Armour::Chestplate) => "Brass Chestplate",
        Material::Brass(Armour::Gauntlets) => "Brass Gauntlets",
        Material::Brass(Armour::Chausses) => "Brass Chausses",
        Material::Brass(Armour::Grieves) => "Brass Grieves",

        _ => "Error",
    }
}
