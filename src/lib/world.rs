use core::any::*;
use std::{
    collections::HashMap,
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u64);
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub trait Component: Any {}

pub struct Components(HashMap<TypeId, Box<dyn Any>>);
impl Components {
    pub fn new() -> Self {
        Components(HashMap::new())
    }
    pub fn get<COMPONENT: Component>(&self) -> Option<&COMPONENT> {
        self.0
            .get(&TypeId::of::<COMPONENT>())
            .map_or(None, |component_ptr| component_ptr.downcast_ref())
    }
    pub fn set<COMPONENT: Component>(&mut self, component: COMPONENT) {
        self.0
            .insert(TypeId::of::<COMPONENT>(), Box::new(component));
    }
}

pub struct World {
    entities: Vec<Entity>,
    components: HashMap<Entity, Components>,
    systems: Vec<fn(&Entity, &Components)>,
}

impl World {
    pub fn new() -> Self {
        World {
            systems: Vec::new(),
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    fn create_entity(&mut self) -> Entity {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let entity = Entity(id);
        let components = Components::new();

        self.entities.push(entity.clone());
        self.components.insert(entity.clone(), components);

        entity
    }

    pub fn set_component<COMPONENT: Component>(&mut self, entity: &Entity, component: COMPONENT) {
        if let Some(components) = self.components.get_mut(entity) {
            components.set(component);
        }
    }

    pub fn spawn<COMPONENT: Component + Any>(&mut self, components: Vec<COMPONENT>) {
        let entity = self.create_entity();
        for component in components {
            self.set_component(&entity, component);
        }
    }

    pub fn add_system(&mut self, system: fn(&Entity, &Components)) {
        self.systems.push(system);
    }

    pub fn update(&self) {
        for system in &self.systems {
            for entity in &self.entities {
                if let Some(components) = self.components.get(&entity) {
                    system(&entity, components);
                }
            }
        }
    }
}
