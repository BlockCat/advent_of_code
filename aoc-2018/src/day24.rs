use std::cell::RefCell;
use std::cmp::Reverse;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Damage {
    Fire, Cold, Radiation, Slashing, Bludgeoning
}

impl From<&str> for Damage {
    fn from(f: &str) -> Self {
        match f {
            "fire" => Damage::Fire,
            "cold" => Damage::Cold,
            "radiation" => Damage::Radiation,
            "slashing" => Damage::Slashing,
            "bludgeoning" => Damage::Bludgeoning,
            _ => unreachable!()
        }
    }
}

#[derive(Clone)]
struct Group {
    id: String,
    is_immune: bool,
    attack: u32,
    damage: Damage,
    units: RefCell<u32>,
    hp: u32,
    weak: Vec<Damage>,
    immune: Vec<Damage>,
    initiative: u32
}

impl Group {
    fn is_alive(&self) -> bool {
        *self.units.borrow() > 0
    }

    fn effective_power(&self) -> u32 {
        *self.units.borrow() * self.attack
    }

    fn get_damage_modifier(&self, damage: &Damage) -> u32 {
        if self.weak.contains(damage) {
            2
        } else if self.immune.contains(damage) {
            0
        } else {
            1
        } 
    }
}

pub fn execute_exercises() {
    let (immune, infection) = parse_input(include_str!("../input/day24_in.txt"));
    println!("Units left of the winning team: {:?}", exercise_1(immune.clone(), infection.clone()));
    println!("Boost needed and units then left: {:?}", exercise_2(immune.clone(), infection.clone()));
}

fn parse_input(input: &str) -> (Vec<Group>, Vec<Group>) {
    
    let mut iter = input.lines();
    iter.next();

    let immune = iter.by_ref().enumerate().take_while(|s| s.1 != "Infection:").map(|(i, l)| parse_line(format!("Immune {}", i+1), l, true)).collect();
    let infection = iter.enumerate().map(|(i, l)| parse_line(format!("Infection {}", i+1), l, false)).collect();    

    (immune, infection)
}

fn parse_line(id: String, line: &str, is_immune: bool) -> Group {        
    let mut line = line.split(' ');
    let units = line.next().unwrap().parse().unwrap();
    line.next();
    line.next();
    line.next();
    let hp = line.next().unwrap().parse().unwrap();

    let mut weak = Vec::with_capacity(2);
    let mut immune = Vec::with_capacity(2);

    line.next();
    line.next();

    'l: loop {
        let mut n = line.next().unwrap();

        if &n[0..1] == "(" {
            n = &n[1..];
        }
                
        match n {
            "weak" => {
                line.next();
                while let Some(e) = line.next() {              
                    weak.push(Damage::from(&e[0..(e.len()-1)]));
                    if e.ends_with(";") || e.ends_with(")") {
                        break;
                    }                    
                }
            }
            "immune" => {
                line.next();
                while let Some(e) = line.next() {                        
                    immune.push(Damage::from(&e[0..(e.len()-1)]));
                    if e.ends_with(";") || e.ends_with(")") {
                        break;
                    }
                }
            }
            _ => {
                line.next();
                line.next();
                line.next();                
                line.next();
                break;
            }
        }
    }

    let attack = line.next().unwrap().parse().unwrap();    
    let damage = Damage::from(line.next().unwrap());
    
    
    line.next();
    line.next();
    line.next();    
    let initiative = line.next().unwrap().parse().unwrap();

    Group {
        id: id,
        attack: attack,
        is_immune: is_immune,
        damage: damage,
        units: RefCell::new(units),
        hp: hp,
        weak: weak,
        immune: immune,
        initiative: initiative

    }
}

fn exercise_1(mut immunes: Vec<Group>, mut infections: Vec<Group>)-> (bool, u32) {

    loop {
        // Start by finding targets
        // At this point all the items are alive        

        /*for group in &immunes {
            println!("{} has {} units left", group.id, *group.units.borrow());
        }

        for group in &infections {
            println!("{} has {} units left", group.id, *group.units.borrow());
        }*/

        infections.sort_by_key(|s| Reverse((s.effective_power(), s.initiative)));
        immunes.sort_by_key(|s| Reverse((s.effective_power(), s.initiative)));        

        let infection_targets = find_targets(&infections, &immunes);
        let immune_targets = find_targets(&immunes, &infections);        

        // Attack!
        let mut attack_order = immune_targets.iter()
            .chain(infection_targets.iter())
            .filter_map(|(g, t)| {
                if let Some(t) = t {
                    Some((g, t))
                } else {
                    None
                }
            }).collect::<Vec<_>>();

        attack_order.sort_by_key(|(group, _)| Reverse(group.initiative));

        if attack_order.is_empty() {
            return (false, 0);
        }

        for (group, target) in attack_order {
            let mut units = target.units.borrow_mut();
            if *units <= 0 {continue;} //Check if current group is still alive
            
            let damage = group.effective_power() * target.get_damage_modifier(&group.damage);
            let units_killed = damage / target.hp;            
            *units = (*units).saturating_sub(units_killed);
            //println!("{} attacks  {} for {} killing {} units", group.id, target.id, damage, units_killed);
        }

        // Only keep the alive ones
        immunes = immunes.into_iter().filter(|s| s.is_alive()).collect();
        infections = infections.into_iter().filter(|s| s.is_alive()).collect();

        if immunes.is_empty() {
            return (false, infections.iter().map(|s| *s.units.borrow()).sum());
        }

        if infections.is_empty() {
            return (true, immunes.iter().map(|s| *s.units.borrow()).sum());
        }

        //return 0;
    }
}

fn exercise_1_boosted(mut immunes: Vec<Group>, infections: Vec<Group>, boost: u32) -> (bool, u32) {
    for c in &mut immunes{
        c.attack += boost;
    }

    exercise_1(immunes, infections)
}

fn exercise_2(immunes: Vec<Group>, infections: Vec<Group>) -> (u32, u32) {
    let mut lower_boost = 1;
    let mut upper_boost = 1;

    while !exercise_1_boosted(immunes.clone(), infections.clone(), upper_boost).0 {
        lower_boost = upper_boost;
        upper_boost *= 2;        
    }

    while upper_boost - lower_boost > 1 {
        let average = (lower_boost + upper_boost) / 2;
        let (immune_win, _) = exercise_1_boosted(immunes.clone(), infections.clone(), average);

        if immune_win {
            upper_boost = average;
        } else {
            lower_boost = average;
        }
    }

    let (_, left) = exercise_1_boosted(immunes.clone(), infections.clone(), upper_boost);

    (upper_boost, left)
}

fn find_targets<'a, 'b>(currents: &'a Vec<Group>, other: &'b Vec<Group>) -> Vec<(&'a Group, Option<&'b Group>)> {
    let mut selected = vec!(false; other.len());    
    currents.iter().map(|s| {            
        let target = other.iter()
            .enumerate()
            .filter(|(i, o)| !selected[*i] &&  o.get_damage_modifier(&s.damage) > 0)            
            .max_by_key(|(_, o)| (s.effective_power() * o.get_damage_modifier(&s.damage), o.effective_power(), o.initiative));
        if let Some(target) = target {
            selected[target.0] = true;
            (s, Some(target.1))
        } else {
            (s, None)
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day24_ex1_s1() {
        let input = r"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3
Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let (immune, infection) = parse_input(input);

        let result = exercise_1(immune, infection);        

        assert_eq!(result, (false, 5216));
    }

    #[test]
    fn day24_ex1_s2() {
        let (immune, infection) = parse_input(include_str!("../input/day24_in.txt"));
        assert_eq!(exercise_1(immune, infection), (false, 16006));
    }

    #[test]
    fn day24_ex2_s1() {
        let input = r"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3
Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let (immune, infection) = parse_input(input);      

        let result = exercise_2(immune, infection);

        assert_eq!(result, (1570, 51));
    }

    #[test]
    fn day24_ex2_s2() {
        let (immune, infection) = parse_input(include_str!("../input/day24_in.txt"));
        assert_eq!(exercise_2(immune, infection), (46, 6221));
    }

    #[bench]    
    fn day24_bench_read(b: &mut Bencher) {        
        b.iter(|| parse_input(include_str!("../input/day24_in.txt")));
    }

    #[bench]
    fn day24_bench_ex1(b: &mut Bencher) {
        let (immune, infection) = parse_input(include_str!("../input/day24_in.txt"));
        b.iter(move || exercise_1(immune.clone(), infection.clone()));
    }

    #[bench]
    fn day24_bench_ex2(b: &mut Bencher) {
        let (immune, infection) = parse_input(include_str!("../input/day24_in.txt"));
        b.iter(move || exercise_2(immune.clone(), infection.clone()));
    }
}