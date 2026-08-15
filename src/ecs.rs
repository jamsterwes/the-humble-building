use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
};

pub type EntityId = u64;
pub type ComponentType = TypeId;
pub trait System {
    type Component: Any;
    fn run(&self, eid: EntityId, component: &mut Self::Component);
}

// Evil fuckery shh
trait InternalSystem {
    fn run_internal(&self, eid: EntityId, component: &mut dyn Any);
}
impl<S: System> InternalSystem for S {
    fn run_internal(&self, eid: EntityId, component: &mut dyn Any) {
        if let Some(c) = component.downcast_mut::<S::Component>() {
            self.run(eid, c);
        } else {
            panic!("run_internal attempted to run a system on the wrong component!!!");
        }
    }
}

pub struct Game {
    _next_entity_id: EntityId,
    _entities: HashSet<EntityId>,
    _entity_components: HashMap<ComponentType, HashMap<EntityId, Box<dyn Any>>>,
    _systems: HashMap<ComponentType, Vec<Box<dyn InternalSystem>>>,
    pub global_entity: EntityId,
}

impl Game {
    pub fn new() -> Self {
        let mut new_game = Self {
            _next_entity_id: 0,
            _entities: HashSet::new(),
            _entity_components: HashMap::new(),
            _systems: HashMap::new(),
            global_entity: 0,
        };
        new_game.global_entity = new_game.new_entity();
        return new_game;
    }

    pub fn new_entity(&mut self) -> EntityId {
        let new_entity_id = self._next_entity_id;
        self._entities.insert(new_entity_id);
        self._next_entity_id += 1;
        return new_entity_id;
    }

    pub fn add_component<T: Any>(&mut self, eid: EntityId, component: T) -> bool {
        if !self._entities.contains(&eid) {
            print!(
                "Error in add_component<{}>: entity {} does NOT exist.",
                std::any::type_name::<T>(),
                eid
            );
            return false;
        }
        let component_type = TypeId::of::<T>();
        let component_map = self
            ._entity_components
            .entry(component_type)
            .or_insert_with(HashMap::new);
        if component_map.contains_key(&eid) {
            print!(
                "Error in add_component<{}>: entity {} already has component of this type!",
                std::any::type_name::<T>(),
                eid
            );
            return false;
        }
        component_map.insert(eid, Box::new(component));
        return true;
    }

    pub fn add_global_component<T: Any>(&mut self, component: T) -> bool {
        self.add_component::<T>(self.global_entity, component)
    }

    pub fn get_component<T: Any>(&self, eid: EntityId) -> Option<&T> {
        self._entity_components
            .get(&TypeId::of::<T>())?
            .get(&eid)?
            .downcast_ref::<T>()
    }

    pub fn get_global_component<T: Any>(&self) -> Option<&T> {
        self.get_component::<T>(self.global_entity)
    }

    pub fn get_component_mut<T: Any>(&mut self, eid: EntityId) -> Option<&mut T> {
        let component_type = TypeId::of::<T>();
        self._entity_components
            .get_mut(&component_type)?
            .get_mut(&eid)?
            .downcast_mut::<T>()
    }

    pub fn get_global_component_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.get_component_mut::<T>(self.global_entity)
    }

    pub fn get_first_entity_with<T: Any>(&self) -> Option<EntityId> {
        self._entity_components
            .get(&TypeId::of::<T>())?
            .keys()
            .next()
            .copied()
    }

    pub fn get_components<T: Any>(&self) -> impl Iterator<Item = (EntityId, &T)> {
        let component_type = TypeId::of::<T>();
        self._entity_components
            .get(&component_type)
            .into_iter()
            .flat_map(|m| {
                m.iter()
                    .map(|(&eid, c)| (eid, c.downcast_ref::<T>().unwrap()))
            })
    }

    pub fn get_components_mut<T: Any>(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        let component_type = TypeId::of::<T>();
        self._entity_components
            .get_mut(&component_type)
            .into_iter()
            .flat_map(|m| {
                m.iter_mut()
                    .map(|(&eid, c)| (eid, c.downcast_mut::<T>().unwrap()))
            })
    }

    pub fn register_system<S: System + 'static>(&mut self, system: S) {
        let component_type = TypeId::of::<S::Component>();
        self._systems
            .entry(component_type)
            .or_insert_with(Vec::new)
            .push(Box::new(system));
    }

    fn run_system<S: System + 'static>(&mut self, system: &S) {
        for (eid, component) in self.get_components_mut::<S::Component>() {
            system.run_internal(eid, component);
        }
    }

    pub fn run_systems(&mut self) {
        for (cty, context_systems) in self._systems.iter() {
            for system in context_systems {
                for (eid, component) in self
                    ._entity_components
                    .entry(*cty)
                    .or_insert_with(HashMap::new)
                    .iter_mut()
                {
                    system.run_internal(*eid, component.as_mut());
                }
            }
        }
    }
}
