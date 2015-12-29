use std::u64;

pub fn main() {
    let invs = all_invs();

    let mut min_so_far = u64::MAX;
    for inv in invs {
        let cost = inv.get_cost();
        if inv.we_win(1, 8, 104) && cost < min_so_far {
            min_so_far = cost;
        }
    }

    println!("{}", min_so_far);
}

#[derive(Debug)]
struct Weapon {
    cost: u64,
    damage: u64,
}

#[derive(Debug)]
struct Armor {
    cost: u64,
    armor: u64,
}

#[derive(Debug)]
struct Ring {
    cost: u64,
    damage: u64,
    armor: u64,
}

// Weapons
static DAGGER : Weapon = Weapon { cost: 8, damage: 4 };
static SHORTSWORD : Weapon = Weapon { cost: 10, damage: 5 };
static WARHAMMER : Weapon = Weapon { cost: 25, damage: 6 };
static LONGSWORD : Weapon = Weapon { cost: 40, damage: 7 };
static GREATAXE : Weapon = Weapon { cost: 74, damage: 8 };

static ALL_WEAPONS : [&'static Weapon; 5] =
    [&DAGGER, &SHORTSWORD, &WARHAMMER, &LONGSWORD, &GREATAXE];

// Armors
static PLACEHOLDER_ARMOR : Armor = Armor { cost: 0, armor: 0 };
static LEATHER : Armor = Armor { cost: 13, armor: 1 };
static CHAINMAIL : Armor = Armor { cost: 31, armor: 2 };
static SPLINTMAIL : Armor = Armor { cost: 53, armor: 3 };
static BANDEDMAIL : Armor = Armor { cost: 75, armor: 4 };
static PLATEMAIL : Armor = Armor { cost: 102, armor: 5 };

static ALL_ARMORS : [&'static Armor; 6] =
    [&PLACEHOLDER_ARMOR, &LEATHER, &CHAINMAIL, &SPLINTMAIL, &BANDEDMAIL, &PLATEMAIL];

// Rings
static PLACEHOLDER_RING : Ring = Ring { cost: 0, damage: 0, armor: 0 };
static DAMAGE1 : Ring = Ring { cost: 25, damage: 1, armor: 0 };
static DAMAGE2 : Ring = Ring { cost: 50, damage: 2, armor: 0 };
static DAMAGE3 : Ring = Ring { cost: 100, damage: 3, armor: 0 };
static DEFENSE1 : Ring = Ring { cost: 20, damage: 0, armor: 1 };
static DEFENSE2 : Ring = Ring { cost: 40, damage: 0, armor: 2 };
static DEFENSE3 : Ring = Ring { cost: 80, damage: 0, armor: 3 };

static ALL_RINGS : [&'static Ring; 7] =
    [&PLACEHOLDER_RING, &DAMAGE1, &DAMAGE2, &DAMAGE3,
     &DEFENSE1, &DEFENSE2, &DEFENSE3];

#[derive(Debug)]
struct Inventory<'a> {
    weapon: &'a Weapon,
    armor: &'a Armor,
    ring1 : &'a Ring,
    ring2 : &'a Ring,
}

// Here I simply generate all the possible inventories. Ideally this would be a
// lazy list/generator etc., but I don't know how to do this in Rust yet.
fn all_invs<'a>() -> Vec<Inventory<'a>> {
    let mut invs : Vec<Inventory<'a>> =
        Vec::with_capacity(ALL_WEAPONS.len() * ALL_ARMORS.len() * ALL_RINGS.len() * ALL_RINGS.len());

    for weapon in ALL_WEAPONS.iter() {
        for armor in ALL_ARMORS.iter() {
            for ring1 in ALL_RINGS.iter() {
                for ring2 in ALL_RINGS.iter() {
                    invs.push(Inventory{ weapon: weapon,
                                         armor: armor,
                                         ring1: ring1,
                                         ring2: ring2 });
                }
            }
        }
    }

    invs
}

impl <'a> Inventory<'a> {
    fn get_damage(&self) -> u64 {
        self.weapon.damage + self.ring1.damage + self.ring2.damage
    }

    fn get_armor(&self) -> u64 {
        self.armor.armor + self.ring1.armor + self.ring2.armor
    }

    fn get_cost(&self) -> u64 {
        self.weapon.cost + self.armor.cost + self.ring1.cost + self.ring2.cost
    }

    fn self_dies_in(&self, enemy_damage : u64) -> u64 {
        f64::ceil(100.0 / ((enemy_damage - self.get_armor()) as f64)) as u64
    }

    fn enemy_dies_in(&self, enemy_armor : u64, enemy_health : u64) -> u64 {
        f64::ceil((enemy_health as f64) / ((self.get_damage() - enemy_armor) as f64)) as u64
    }

    fn we_win(&self, enemy_armor : u64, enemy_damage : u64, enemy_health : u64) -> bool {
        self.enemy_dies_in(enemy_armor, enemy_health) <= self.self_dies_in(enemy_damage)
    }
}
