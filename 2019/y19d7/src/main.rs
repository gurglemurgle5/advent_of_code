use aoc_utils::intcode::Intcode;

const AMPS: usize = 5;

fn main() {
    let input = Intcode::read_to_vec("./input.txt");
    let program = Intcode::new(input);

    let mut best = 0;

    let mut phases = [0i64; AMPS];
    'phases: loop {
        for phase in &mut phases {
            *phase += 1;
            if *phase == 5 {
                *phase = 0;
            } else {
                break;
            }
        }
        if phases == [0; AMPS] {
            break;
        }
        for left in 0..phases.len() - 1 {
            for right in (left + 1)..phases.len() {
                if phases[left] == phases[right] {
                    continue 'phases;
                }
            }
        }
        let mut amps = Vec::new();
        for phase in phases {
            let mut program = program.clone();
            program.step_until_done();
            program.input(phase);
            program.step_until_done();
            amps.push(program);
        }

        amps[0].input(0);
        amps[0].step_until_done();

        for i in 0..(amps.len() - 1) {
            let output = amps[i].output();
            amps[i + 1].input(output);
            amps[i].step_until_done();
            amps[i + 1].step_until_done();
        }

        let output = amps.last_mut().unwrap().output();
        amps.last_mut().unwrap().step_until_done();
        if output > best {
            best = output;
        }
    }

    dbg!(best);
}
