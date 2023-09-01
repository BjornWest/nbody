use rand::Rng;
use std::thread;
use std::sync::Arc;

#[derive(Clone, Copy)]
struct Entity {
    x: f64,
    y: f64,
    mass: f64,
    velocity_x: f64,
    velocity_y: f64,
}

const N: i32 = 3000;
const DELTA: f64 = 1e-5;
const EPSILON: f64 = 1e-3;

fn main() {
    let binding: Vec<Entity> = spawn_data();
    let mut entities: Arc<[Entity]> = binding.into();
    let timesteps: i32 = 200;
    let gravity = 100.0 / entities.len() as f64;

    for x in 0..timesteps {
        println!("{x}");
        entities = performstep_multi(entities, gravity, EPSILON);
    }
}

fn spawn_data() -> Vec<Entity> {
    let mut entities = Vec::new();
    for _x in 0..N {
        entities.push(Entity {
            x: rand::thread_rng().gen_range(0..10) as f64 / 10.0,
            y: rand::thread_rng().gen_range(0..10) as f64 / 10.0,
            mass: rand::thread_rng().gen_range(0..10) as f64,
            velocity_x: rand::thread_rng().gen_range(0..10) as f64,
            velocity_y: rand::thread_rng().gen_range(0..10) as f64,
        });
    }
    entities
}

fn distance(elem1: &Entity, elem2: &Entity) -> f64 {
    let x: f64 = elem1.x - elem2.x;
    let y: f64 = elem1.y - elem2.y;
    return (x * x + y * y).sqrt();
}

fn performstep(old_entities: &Vec<Entity>, gravity: f64, epsilon: f64) -> Vec<Entity> {
    let mut new_entities: Vec<Entity> = old_entities.clone();

    for new_entity in &mut new_entities {
        let mut a = vec![0.0, 0.0];
        for old_entity in old_entities {
            let r = distance(&new_entity, &old_entity);
            let factor = old_entity.mass * (epsilon + r).powf(-3.0);
            a[0] = factor * (new_entity.x - old_entity.x);
            a[1] = factor * (new_entity.y - old_entity.y);
        }
        a[0] *= -gravity;
        a[1] *= -gravity;
        new_entity.velocity_x += DELTA * a[0];
        new_entity.velocity_y += DELTA * a[1];

        new_entity.x += DELTA * new_entity.velocity_x;
        new_entity.y += DELTA * new_entity.velocity_y;
    }
    return new_entities;
}

fn performstep_multi(old_entities: Arc<[Entity]>, gravity: f64, epsilon: f64) -> Arc<[Entity]> {
    let new_entities: Arc<[Entity]> = old_entities.clone();

    //let new_entities = RefCell::new(new_entities);

    let mut handles = Vec::new();
    for new_entity in new_entities.iter() {
        let old_copy = old_entities.clone();
        let new = new_entity.to_owned();
        let handle = thread::spawn(move ||{
            thread_step(new, old_copy, gravity, epsilon);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    return new_entities;
}

fn thread_step(mut new_entity: Entity, old_entities: Arc<[Entity]>, gravity: f64, epsilon: f64) -> Entity{
    let mut a = (0.0, 0.0);
    for old_entity in old_entities.iter() {
        let r = distance(&new_entity, &old_entity);
        let factor = old_entity.mass * (epsilon + r).powf(-3.0);
        a.0 = factor * (new_entity.x - old_entity.x);
        a.1 = factor * (new_entity.y - old_entity.y);
    }
    a.0 *= -gravity;
    a.1 *= -gravity;
    new_entity.velocity_x += DELTA * a.0;
    new_entity.velocity_y += DELTA * a.1;

    new_entity.x += DELTA * new_entity.velocity_x;
    new_entity.y += DELTA * new_entity.velocity_y;
    return new_entity;
}
