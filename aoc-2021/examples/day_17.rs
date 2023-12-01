use aoc_2021::vector::Vector2;

type Input = ((isize, isize), (isize, isize));

pub fn main() {
    // let input = ((117, 7310), (-9546, -89));
    let input = ((281, 311), (-74, -54));

    println!("Ex1: {:?}", exercise_1(&input));
    println!("Ex2: {:?}", exercise_2(&input));
}


fn shoot_probe_y(mut pos: isize, mut vel: isize, area: &(isize, isize)) -> Option<isize> {    

    // f(t) = (vel) + (vel - 1) + (vel - 2) + ... + (vel - t + 1) + (vel - t)
    // f(t) = (vel - t) + (vel - t + 1) ... + (vel - 2) + (vel - 1) + (vel)
    // f(t) = (2 vel - t) * t / 2
    

    // area.0 >= (2 * vel - t) * t / 2
    // 2 * area.0 >= (2 * vel - t) * t = 2 * vel * t - t*t
    // 0 >= (2 * vel - t) * t = 2 * vel * t - t*t - 2 * area.0

    while pos >= area.0 {
        pos += vel;
        vel -= 1;

        if (area.0..=area.1).contains(&pos) {            
            return Some(pos);
        }
    }
    None
}

fn shoot_probe_xy(
    mut pos: Vector2,
    mut vel: Vector2,
    area_x: &(isize, isize),
    area_y: &(isize, isize),
) -> Option<Vector2> {
    while pos[0] <= area_x.1 && pos[1] >= area_y.0 {
        pos += vel;

        if vel[0] > 0 {
            vel += Vector2::new([-1, 0]);
        }

        vel += Vector2::new([0, -1]);

        if (area_x.0..=area_x.1).contains(&pos[0]) && (area_y.0..=area_y.1).contains(&pos[1]) {
            return Some(pos);
        }
    }

    None
}

fn exercise_1(input: &Input) -> isize {
    let y = (0..=10_000)
        .rev()
        .filter_map(|x| shoot_probe_y(0, x, &input.1).map(|a| x))
        .next()
        .unwrap();
    y * (y + 1) / 2
}

fn exercise_2(input: &Input) -> usize {
    let min_x: isize = 1;
    let max_x: isize = input.0 .1 + 1;
    let min_y = -input.1.0.abs();
    let max_y = input.1.0.abs();
    let mut counter = 0;
    for dx in min_x..max_x {
        for dy in min_y..max_y {
            if shoot_probe_xy(
                Vector2::new([0, 0]),
                Vector2::new([dx, dy]),
                &input.0,
                &input.1,
            )
            .is_some()
            {
                counter += 1;
            }
        }
    }

    counter
}
