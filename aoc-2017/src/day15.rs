const REMAINDER: u64 = 2147483647;
struct Machine {
    factor: u64,
    value: u64,
    multiple: u64,
}

impl Machine {
    fn new(factor: u64, value: u64, multiple: u64) -> Machine {
        Machine { factor, value, multiple }
    }
}


impl Iterator for Machine {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.multiple == 1 {
            let temp = self.factor * self.value;
            let temp = temp % REMAINDER;
            self.value = temp;

            Some(temp)
        } else {
            while {
                let temp = self.factor * self.value;
                let temp = temp % REMAINDER;
                self.value = temp;
                (temp & (self.multiple - 1)) != 0
            } {}
            return Some(self.value);
        }
    }
}

fn algorithm1(a: u64, b: u64) -> usize {    
    let machine_a = Machine::new(16807, a, 1);
    let machine_b = Machine::new(48271, b, 1);

    machine_a.zip(machine_b).take(40_000_000).filter(|(a, b)| (a & 65535) == (b & 65535)).count()
}

fn algorithm2(a: u64, b: u64) -> usize { 
    let machine_a = Machine::new(16807, a, 4);
    let machine_b = Machine::new(48271, b, 8);

    machine_a.zip(machine_b)
        .take(5_000_000)
        .filter(|(a, b)| (a & 65535) == (b & 65535)).count()
}

#[test]
fn run15() {
    println!("first: {}, second: {}", algorithm1(634, 301), algorithm2(634, 301));

}

#[test]
fn test_examples() {    
    let mut machine_a = Machine::new(16807, 65, 1);
    let mut machine_b = Machine::new(48271, 8921, 1);

    assert_eq!(algorithm1(65, 8921), 588);
    assert_eq!(algorithm2(65, 8921), 309);
}

#[test]
fn test_sequence() {
    let mut machine_a = Machine::new(16807, 65, 1);
    let mut machine_b = Machine::new(48271, 8921, 1);

    assert_eq!(machine_a.next(), Some(1092455));
    assert_eq!(machine_a.next(), Some(1181022009));
    assert_eq!(machine_a.next(), Some(245556042));
    assert_eq!(machine_a.next(), Some(1744312007));
    assert_eq!(machine_a.next(), Some(1352636452));

    assert_eq!(machine_b.next(), Some(430625591));
    assert_eq!(machine_b.next(), Some(1233683848));
    assert_eq!(machine_b.next(), Some(1431495498));
    assert_eq!(machine_b.next(), Some(137874439));
    assert_eq!(machine_b.next(), Some(285222916));

    let mut machine_a = Machine::new(16807, 65, 4);
    let mut machine_b = Machine::new(48271, 8921, 8);

    assert_eq!(machine_a.next(), Some(1352636452));
    assert_eq!(machine_a.next(), Some(1992081072   ));
    assert_eq!(machine_a.next(), Some(530830436  ));
    assert_eq!(machine_a.next(), Some(1980017072  ));
    assert_eq!(machine_a.next(), Some(740335192   ));

    assert_eq!(machine_b.next(), Some(1233683848));
    assert_eq!(machine_b.next(), Some(862516352));
    assert_eq!(machine_b.next(), Some(1159784568));
    assert_eq!(machine_b.next(), Some(1616057672));
    assert_eq!(machine_b.next(), Some(412269392));
}

