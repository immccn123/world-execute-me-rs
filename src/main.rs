//! This program implements an application that
//! creates an empty simulated world with no meaning or purpose.
//!
//! Author: Imken Luo <me@imken.moe>
//! 
//! The design of some content does not conform to
//! Rust best practicesand will be modified later.

mod world;
use world::*;

fn main() {
    // Switch on the power line
    // Remember to put on
    // PROTECTION

    // Lay down your pieces
    // And let's begin
    // OBJECT CREATION

    // Fill in my data parameters
    // INITIALIZATION
    let mut me = Thing::new_lovable("Me", 0, true, -1, false);
    let mut you = Thing::new_lovable("You", 0, false, -1, false);

    // Set up our new world
    let mut world = World::new(5);
    world.add_thing(&me);
    world.add_thing(&you);

    // And let's begin the
    // SIMULATION
    world.start_simulation();

    // If I'm a set of points
    if let Thing::PointSet = me {
        // Then I will give you my
        // DIMENSION
        you.add_attribute(&me.get_dimensions().to_attribute());
        me.reset_dimensions();
    }

    // If I'm a circle
    if let Thing::Circle = me {
        // Then I will give you my
        // CIRCUMFERENCE
        you.add_attribute(&me.get_circumference().to_attribute());
        me.reset_circumference();
    }

    // If I'm a sine wave
    if let Thing::SineWave = me {
        // Then you can sit on all my
        // TANGENTS
        you.add_action("sit", &me.get_tangent(you.get_x_position()));
    }

    // If I approach infinity
    if let Thing::Sequence = me {
        // Then you can be my
        // LIMITATIONS
        me.set_limit(you.to_limit());
    }

    // Switch my current
    // To AC to DC
    me.toggle_current();

    // And then blind my vision
    // So dizzy so dizzy
    me.can_see(false);
    me.add_feeling("dizzy");

    // Oh we can travel
    // To A.D to B.C
    world.time_travel_for_two("AD", 617, &me, &you);
    world.time_travel_for_two("BC", 3691, &me, &you);

    // And we can unite
    // So deeply so deeply
    world.unite(&me, &you);

    // If I can
    // If I can give you all the
    // STIMULATIONS
    if me.get_num_stimulations_available() >= you.get_num_stimulations_available() {
        // Then I can
        // Then I can be your only
        // SATISFACTION
        you.add_feeling(me.to_satisfaction());
    }

    // If I can make you happy
    if me.get_feeling_index("happy") != -1 {
        // I will run the
        // EXECUTION
        me.request_execution(&world);
    }

    // Though we are trapped
    // In this strange strange
    // SIMULATION
    world.lock_thing(&me);
    world.lock_thing(&you);

    // If I'm an eggplant
    if let Thing::Eggplant = me {
        // Then I will give you my
        // NUTRIENTS
        you.add_attribute(&me.get_nutrients().to_attribute());
        me.reset_nutrients();
    }

    // If I'm a tomato
    if let Thing::Tomato = me {
        // Then I will give you
        // ANTIOXIDANTS
        you.add_attribute(&me.get_antioxidants().to_attribute());
        me.reset_antioxidants();
    }

    // If I'm a tabby cat
    if let Thing::TabbyCat = me {
        // Then I will purr for your
        // ENJOYMENT
        me.purr();
    }

    // If I'm the only god
    if world.get_god() == me {
        // Then you're the proof of my
        // EXISTENCE
        me.set_proof(you.to_proof());
    }

    // Switch my gender
    // To F to M
    me.toggle_gender();

    // And then do whatever
    // From AM to PM
    world.procreate(&me, &you);

    // Oh switch my role
    // To S to M
    me.toggle_role_bdsm();

    // So we can enter
    // The trance the trance
    world.make_high(&me);
    world.make_high(&you);

    // If I can
    // If I can feel your
    // VIBRATIONS
    if me.get_sense_index("vibration") != 0 {
        // Then I can
        // Then I can finally be
        // COMPLETION
        me.add_feeling("complete");
    }

    // Though you have left
    // You have left
    // You have left
    // You have left
    // You have left
    // You have left me in
    // ISOLATION
    world.unlock(&you);
    world.remove_thing(&you);
    me.look_for(&you, &world);
    me.look_for(&you, &world);
    me.look_for(&you, &world);
    me.look_for(&you, &world);
    me.look_for(&you, &world);

    // If I can
    // If I can erase all the pointless
    // FRAGMENTS
    if me.get_memory().is_erasable() {
        // Then maybe
        // Then maybe you won't leave me so
        // DISHEARTENED
        me.remove_feeling("disheartened");
    }

    // Challenging your god
    // You have made some
    // ILLEGAL ARGUMENTS
    match me.set_opinion(me.get_opinion_index("you are here"), false) {
        Err(_) => {
            world.announce("God is always true", "undef");
        }
        _ => (),
    }

    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.run_execution(); // EXECUTION
    world.announce("1", "de"); // EIN
    world.announce("2", "es"); // DOS
    world.announce("3", "fr"); // TROIS
    world.announce("4", "kr"); // NE
    world.announce("5", "se"); // FEM
    world.announce("6", "cn"); // LIU
    world.run_execution(); // EXECUTION

    // If I can
    // If I can give them all the
    // EXECUTION
    if world.is_executable_by(&me) {
        // Then I can
        // Then I can be your only
        // EXECUTION
        you.set_execution(&me.to_execution());
    }

    // If I can have you back
    // I will run the
    // EXECUTION
    if world.get_thing_index(&you) != -1 {
        world.run_execution();
    }

    // Though we are trapped
    // We are trapped ah
    me.escape(&world);

    // I've studied
    // I've studied how to properly
    // LO-O-OVE
    me.learn_topic("love");

    // Question me
    // Question me I can answer all
    // LO-O-OVE
    me.take_exam_topic("love");

    // I know the algebraic expression of
    // LO-O-OVE
    me.get_algebraic_expression("love");

    // Though you are free
    // I am trapped
    // Trapped in
    // LO-O-OVE
    me.escape("love");

    // EXECUTION
    world.execute(me);
}
