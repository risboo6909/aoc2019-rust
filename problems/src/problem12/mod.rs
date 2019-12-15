use std::fmt;
use failure::Error;

use utils::{split_by_lines, result, ProblemResult, RetOne,};
use num_integer::Integer;

const SIM_STEPS: usize = 1000;
const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

#[derive(Debug, Default, Copy, Clone)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct IterDelta {
    iter: usize,
    delta: usize,
    len: usize,
    found: bool,
}

fn compute_energy(objects: &[Point3], vel: &[Point3]) -> usize {
    let mut energy = 0;

    // compute energy
    for (idx, obj) in objects.iter().enumerate() {
        let pot = obj.x.abs() + obj.y.abs() + obj.z.abs();
        let kin = vel[idx].x.abs() + vel[idx].y.abs() + vel[idx].z.abs();

        energy += (pot * kin) as usize;
    }

    energy
}

fn update_vel(objects: &[Point3], vel: &mut [Point3], idx1: usize, idx2: usize) {

    let obj1 = objects[idx1];
    let obj2 = objects[idx2];

    if obj1.x > obj2.x {
        vel[idx1].x -= 1;
        vel[idx2].x += 1;
    } else if obj1.x < obj2.x {
        vel[idx1].x += 1;
        vel[idx2].x -= 1;
    }

    if obj1.y > obj2.y {
        vel[idx1].y -= 1;
        vel[idx2].y += 1;
    } else if obj1.y < obj2.y {
        vel[idx1].y += 1;
        vel[idx2].y -= 1;
    }

    if obj1.z > obj2.z {
        vel[idx1].z -= 1;
        vel[idx2].z += 1;
    } else if obj1.z < obj2.z {
        vel[idx1].z += 1;
        vel[idx2].z -= 1;
    }
}

fn update_coords(objects: &mut [Point3], vel: &[Point3]) {
    // apply velocities
    for (idx, obj) in objects.iter_mut().enumerate() {
        obj.x += vel[idx].x;
        obj.y += vel[idx].y;
        obj.z += vel[idx].z;
    }
}

fn first_star(mut objects: &mut Vec<Point3>) -> ProblemResult<usize> {

    let mut vel = vec![Point3::default(), Point3::default(), Point3::default(), Point3::default()];

    for _ in 0..SIM_STEPS {

        // apply gravity
        for (idx1, _) in objects.iter().enumerate() {
            for (idx2, _) in objects.iter().enumerate().skip(idx1 + 1) {
                update_vel(&objects, &mut vel, idx1, idx2);
            }
        }

        // apply velocities
        update_coords(&mut objects, &vel);

    }

    let energy = compute_energy(&objects, &vel);

    Ok(energy)

}

fn update_component(iter: usize, e: &mut IterDelta) {

    if e.found {
        return
    }

    let delta = iter - e.iter;

    if delta == e.delta {
        e.found = true;
    } else {
        e.iter = iter + 1;
        e.len += 1;
        if e.delta == 0 {
            e.delta = delta
        }
    }

}

fn second_star(mut objects: &mut Vec<Point3>) -> ProblemResult<usize> {

    let start_pos = objects.clone();

    let mut iter = 0;

    let mut vel = vec![Point3::default(); objects.len()];
    let mut periods = vec![IterDelta::default(); 3];

    // find periods
    loop {

        // apply gravity
        for (idx1, _) in objects.iter().enumerate() {
            for (idx2, _) in objects.iter().enumerate().skip(idx1 + 1) {
                update_vel(&objects, &mut vel, idx1, idx2);
            }
        }

        // apply velocities
        update_coords(&mut objects, &vel);

        // check periods
        if objects.iter().enumerate().all(|(i, obj)| obj.x == start_pos[i].x && vel[i].x == 0) {
            update_component(iter, &mut periods[X]);
        }

        if objects.iter().enumerate().all(|(i, obj)| obj.y == start_pos[i].y && vel[i].y == 0) {
            update_component(iter, &mut periods[Y]);
        }

        if objects.iter().enumerate().all(|(i, obj)| obj.z == start_pos[i].z && vel[i].z == 0) {
            update_component(iter, &mut periods[Z]);
        }

        if periods.iter().all(|e| e.found) {
            break
        }

        iter += 1;
    }

    let tmp = periods[X].iter * periods[Y].iter / periods[X].iter.gcd(&periods[Y].iter);
    let lcm = tmp * periods[Z].iter / tmp.gcd(&periods[Z].iter);

    Ok(lcm)
}

pub(crate) fn solve() -> Result<RetOne<usize>, Error> {
    let input_raw = include_str!("./input");

    let input = split_by_lines(input_raw, &|line: &str| {

        let trimmed = &line[1..line.len() - 1];
        let mut tmp = Vec::with_capacity(3);

        for part in trimmed.split(',').collect::<Vec<&str>>() {
            tmp.push(part.split('=').collect::<Vec<&str>>()[1].parse::<isize>()?);
        }

        let p = Point3 {
            x: tmp[0],
            y: tmp[1],
            z: tmp[2],
        };

        Ok(p)

    })?;

    Ok(result(first_star(&mut input.clone()), second_star(&mut input.clone())))
}
