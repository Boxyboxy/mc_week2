// 1. create a struct Unicorn with fields:
 // name of type String
 // magic_powers of type u32
#[derive(Debug)] // allows for print formatting
struct Unicorn {
    name: String,
    magic_powers: u32
}
// 2. create a struct Griffin with fields:
 // name of type String
 // magic_powers of type u32
#[derive(Debug)]
struct Griffin {
    name: String,
    magic_powers: u32
}
 // 5. create an enum Creature with Variants:
 // Unicorn with data of (Unicorn)
 // Griffin with data of (Griffin)

#[derive(Debug)]
enum Creature {
    Unicorn(Unicorn),
    Griffin(Griffin)
}

impl Creature {
// 8. implement onto Creature
 // 9. define magic_power function that takes in a referenced self and outputs unsigned 32bits integer
 // matches on itself
 // if its of variant Unicorn, return referenced unicorn's magic power
 // if its of variant Griffin, return referenced griffin's magic power

 // 12. define name function that takes in a referenced self and outputs a string slice
 // matches on itself
 // if its of variant Unicorn, return referenced unicorn's name
 // if its of variant Griffin, return referenced griffin's name
 // 13. define clone function which takes in a referenced self and returns Self/Creature (interchangeable)
 // matches on itself
 // if its of variant Unicorn, return a Unicorn variant
 // name should be assigned to the output of the previously defined name function above
 // magic_power should be assigned to the output of the previously defined magic_power function above
 // if its of variant Griffin, return a Griffin variant
 // name should be assigned to the output of the previously defined name function above
 // magic_power should be assigned to the output of the previously defined magic_power function above
    fn magic_power(&self) -> u32 {
        match self {
            Creature::Unicorn(unicorn) => {
                unicorn.magic_powers
            },
            Creature::Griffin(griffin) => {
                griffin.magic_powers
            }
        }
    }

    fn name(&self) -> &str{
        match self {
            Creature::Unicorn(unicorn) => &unicorn.name,
            Creature::Griffin(griffin) => &griffin.name
        }
    }

    fn clone(&self) -> Creature {
        match self {
            Creature::Unicorn(unicorn) => {
                let new_unicorn = Unicorn {
                    name: unicorn.name.clone(),
                    magic_powers: unicorn.magic_powers
                };
                Creature::Unicorn(new_unicorn)

            },
            Creature::Griffin(griffin) => {
                let new_griffin = Griffin {
                    name:griffin.name.clone(),
                    magic_powers: griffin.magic_powers
                };
                Creature::Griffin(new_griffin)
            }
        }
    }
}

// 10. define compare_magic function, takes in 2 referenced creatures and return 1 referenced creature (lifetime!!)
 // if first creature has larger output derived from magic_power function
 // return first creature
 // else
 // return second creature

 //this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b` help: consider introducing a named lifetime parameter
 fn compare_magic<'a>(a: &'a Creature, b: &'a Creature)-> &'a Creature{
    if a.magic_power() > b.magic_power() {
        a
    } else {
        b
    }

 }
// 14. define creature_box function, takes in 1 referenced creature and returns box with an owned creature (lifetime!!)
 // return a box containing an owned creature by calling the clone function within the box
fn creature_box<'a>(creature: &'a Creature) -> Box<Creature> {
    // return a box containing an owned creature by calling the clone function within the box
    Box::new(creature.clone())
}


fn main() {
    // 3. create a variable unicorn of Unicorn instance
    // 4. create a variable griffin of Griffin instance
    // 6. create variable unicorn_creature, a Creature enum with Unicorn variant
    // 7. create variable griffin_creature, a Creature enum with Griffin variant
    // 11. create stronger_creature, the output of compare_magic function, passing in unicorn and griffin creature
    // 15. create boxed_creature, the output of creature_box function, passing in stronger creature
    // 16. print "The stronger creature is <creature name> with a magic power of <creature power>."

    let unicorn1 = Unicorn {
        name: "charlie".to_owned(),
        magic_powers: 1
    };

    let griffin1 = Griffin{
        name: "harry".to_owned(),
        magic_powers: 2
    };

    let unicorn_creature = Creature::Unicorn(unicorn1);
    let griffin_creature = Creature::Griffin(griffin1);

    let stronger_creature = compare_magic(&unicorn_creature, &griffin_creature);
    println!("{:?}", stronger_creature);
    let boxed_creature = creature_box(stronger_creature);   
    println!("The stronger creature is {} with a magic power of {}", boxed_creature.name(), boxed_creature.magic_power());


}
