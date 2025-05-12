use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (line0, line1) = input.split_once('\n').unwrap();
    let line0 = line0.split(',');
    let line1 = line1.split(',');

    let mut segments0 = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for part in line0 {
        let mut part = part.chars();
        let dir = part.next().unwrap();
        let num: String = part.collect();
        let num: i32 = num.parse().unwrap();

        match dir {
            'U' => {
                segments0.push(((x, y), (x, y + num)));
                y += num;
            }
            'D' => {
                segments0.push(((x, y), (x, y - num)));
                y -= num;
            }
            'L' => {
                segments0.push(((x, y), (x - num, y)));
                x -= num;
            }
            'R' => {
                segments0.push(((x, y), (x + num, y)));
                x += num;
            }
            _ => panic!("unknown dir: {dir}"),
        }
    }
    x = 0;
    y = 0;
    let mut segments1 = Vec::new();
    for part in line1 {
        let mut part = part.chars();
        let dir = part.next().unwrap();
        let num: String = part.collect();
        let num: i32 = num.parse().unwrap();

        match dir {
            'U' => {
                segments1.push(((x, y), (x, y + num)));
                y += num;
            }
            'D' => {
                segments1.push(((x, y), (x, y - num)));
                y -= num;
            }
            'L' => {
                segments1.push(((x, y), (x - num, y)));
                x -= num;
            }
            'R' => {
                segments1.push(((x, y), (x + num, y)));
                x += num;
            }
            _ => panic!("unknown dir: {dir}"),
        }
    }

    let mut closest: (i32, i32) = (0, 0);

    for &segment0 in &segments0 {
        for &segment1 in &segments1 {
            if segment0.0.0 == segment0.1.0 {
                // segment0 is vertical
                if segment1.0.0 == segment1.1.0 {
                    // segment1 is vertical
                    continue;
                }
                // segment1 is horizontal
                let x = segment0.0.0;
                let y = segment1.0.1;
                if x >= segment1.0.0.min(segment1.1.0)
                    && x <= segment1.0.0.max(segment1.1.0)
                    && y >= segment0.0.1.min(segment0.1.1)
                    && y <= segment0.0.1.max(segment0.1.1)
                    && (closest == (0, 0)
                        || (x.abs() + y.abs()) < (closest.0.abs() + closest.1.abs()))
                {
                    closest = (x, y);
                }
            } else {
                // segment0 is horizontal
                if segment1.0.0 == segment1.1.0 {
                    // segment1 is vertical
                    let x = segment1.0.0;
                    let y = segment0.0.1;

                    if x >= segment0.0.0.min(segment0.1.0)
                        && x <= segment0.0.0.max(segment0.1.0)
                        && y >= segment1.0.1.min(segment1.1.1)
                        && y <= segment1.0.1.max(segment1.1.1)
                        && (closest == (0, 0)
                            || (x.abs() + y.abs()) < (closest.0.abs() + closest.1.abs()))
                    {
                        closest = (x, y);
                    }
                }
            }
        }
    }

    dbg!(closest, closest.0.abs() + closest.1.abs());
}
