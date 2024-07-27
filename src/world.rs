#![allow(unused)]

pub enum Thing {
    Lovable {
        name: String,
        x_position: i32,
        lovable: bool,
        current: i32,
        see: bool,
    },
    PointSet,
    Circle,
    SineWave,
    Sequence,
    Eggplant,
    Tomato,
    TabbyCat,
}

pub struct Attribute;
pub struct Memory;
pub struct IllegalArgumentError;

impl PartialEq for Thing {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Thing::Lovable { name: name1, .. }, Thing::Lovable { name: name2, .. }) => {
                name1 == name2
            }
            (Thing::PointSet, Thing::PointSet) => true,
            (Thing::Circle, Thing::Circle) => true,
            (Thing::SineWave, Thing::SineWave) => true,
            (Thing::Sequence, Thing::Sequence) => true,
            (Thing::Eggplant, Thing::Eggplant) => true,
            (Thing::Tomato, Thing::Tomato) => true,
            (Thing::TabbyCat, Thing::TabbyCat) => true,
            _ => false,
        }
    }
}

impl Thing {
    pub fn new_lovable(
        name: &str,
        x_position: i32,
        lovable: bool,
        current: i32,
        see: bool,
    ) -> Self {
        Thing::Lovable {
            name: name.to_string(),
            x_position,
            lovable,
            current,
            see,
        }
    }
    pub fn reset_dimensions(&mut self) {}
    pub fn get_x_position(&self) -> i32 {
        0
    }
    pub fn to_satisfaction(&self) -> &str {
        "satisfied"
    }
    pub fn get_feeling_index(&self, _feeling: &str) -> i32 {
        0
    }
    pub fn reset_circumference(&mut self) {}
    pub fn add_action<T>(&mut self, _name: &str, _action: T) {}
    pub fn toggle_current(&mut self) {
        if let Thing::Lovable { current, .. } = self {
            *current = if *current == 0 { 1 } else { 0 };
        }
    }
    pub fn can_see(&mut self, see: bool) {
        if let Thing::Lovable { see: see_field, .. } = self {
            *see_field = see;
        }
    }
    pub fn add_feeling(&mut self, _feeling: &str) {}
    pub fn add_attribute(&mut self, _attr: &Attribute) {}
    pub fn get_num_stimulations_available(&self) -> i32 {
        0
    }
    pub fn look_for(&self, _you: &Thing, _world: &World) {}
    pub fn remove_feeling(&mut self, _feeling: &str) {}
    pub fn set_opinion(&mut self, _index: i32, _value: bool) -> Result<(), IllegalArgumentError> {
        Ok(())
    }
    pub fn escape<T: ?Sized>(&self, _from: &T) {}
    pub fn learn_topic(&self, _topic: &str) {}
    pub fn take_exam_topic(&self, _topic: &str) {}
    pub fn get_algebraic_expression(&self, _expression: &str) {}
    pub fn get_circumference(&self) {}
    pub fn get_dimensions(&self) {}
    pub fn get_tangent(&self, _x: i32) {}
    pub fn to_limit(&self) -> i32 {
        0
    }
    pub fn set_limit(&mut self, _new_limit: i32) {}
    pub fn to_execution(&self) -> i32 {
        0
    }
    pub fn request_execution(&mut self, _world: &World) {}
    pub fn get_nutrients(&self) {}
    pub fn reset_nutrients(&mut self) {}
    pub fn get_antioxidants(&self) {}
    pub fn reset_antioxidants(&mut self) {}
    pub fn purr(&self) {}
    pub fn set_proof(&mut self, _proof: ()) {}
    pub fn to_proof(&self) {}
    pub fn toggle_gender(&mut self) {}
    pub fn toggle_role_bdsm(&mut self) {}
    pub fn get_sense_index(&self, _name: &str) -> i32 {
        0
    }
    pub fn get_memory(&self) -> Memory {
        Memory
    }
    pub fn get_opinion_index(&self, _name: &str) -> i32 {
        0
    }
    pub fn set_execution(&mut self, _value: &i32) {}
}

pub struct World {
    size: i32,
}

impl World {
    pub fn new(size: i32) -> Self {
        World { size }
    }
    pub fn add_thing(&mut self, _thing: &Thing) {}
    pub fn start_simulation(&self) {}
    pub fn time_travel_for_two(&self, _era: &str, _year: i32, _me: &Thing, _you: &Thing) {}
    pub fn unite(&self, _me: &Thing, _you: &Thing) {}
    pub fn lock_thing(&self, _thing: &Thing) {}
    pub fn run_execution(&self) {}
    pub fn announce(&self, _msg: &str, _lang: &str) {}
    pub fn is_executable_by(&self, _thing: &Thing) -> bool {
        false
    }
    pub fn get_god(&self) -> Thing {
        Thing::new_lovable("God", 0, false, 0, false)
    }
    pub fn execute(&self, _thing: &Thing) {}
    pub fn unlock(&self, _thing: &Thing) {}
    pub fn remove_thing(&self, _thing: &Thing) {}
    pub fn make_high(&self, _thing: &Thing) {}
    pub fn procreate(&self, _me: &Thing, _you: &Thing) {}
    pub fn get_thing_index(&self, _thing: &Thing) -> i32 {
        -1
    }
}

impl Memory {
    pub fn is_erasable(&self) -> bool {
        false
    }
}

pub trait ToAttribute {
    fn to_attribute(&self) -> Attribute;
}

impl<T> ToAttribute for T {
    fn to_attribute(&self) -> Attribute {
        Attribute
    }
}
