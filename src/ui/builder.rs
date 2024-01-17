use bevy::{ecs::system::EntityCommands, prelude::*, utils::all_tuples};

pub struct UiBuilder<'a, 'b, 'c, 'd> {
    pub builder: &'a mut ChildBuilder<'b, 'c, 'd>,
    pub world: &'a World,
}

pub struct UiCommands<'a, 'b, 'c, 'd> {
    commands: EntityCommands<'b, 'c, 'd>,
    world: &'a World,
}

impl<'a, 'b, 'c, 'd> UiBuilder<'a, 'b, 'c, 'd> {
    pub fn spawn(&mut self, bundle: impl Bundle) -> UiCommands<'a, 'b, 'c, '_> {
        let commands: EntityCommands<'b, 'c, '_> = self.builder.spawn(bundle);
        UiCommands {
            commands,
            world: self.world,
        }
    }
}


impl<'a, 'b, 'c, 'd> UiCommands<'a, 'b, 'c, 'd> {
    pub fn id(&self) -> Entity {
        self.commands.id()
    }
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.commands.insert(bundle);
        self
    }
    pub fn with_children(&mut self, spawn_children: impl FnOnce(&mut UiBuilder)) -> &mut Self {
        self.commands.with_children(|builder| {
            let mut ui_builder = UiBuilder {
                builder,
                world: self.world,
            };
            spawn_children(&mut ui_builder);
        });
        self
    }
}

pub trait Class<P> {
    type In;
    fn apply(self, b: &mut Self::In, world: &World);
}

macro_rules! impl_class_tuple {
    ($($P: ident),*) => {
        impl<B, F, $($P),*> Class<(B, $($P,)*)> for F
        where
            F: FnOnce(&mut B, $(& $P), *),
            $($P: Resource,)*
        {
            type In = B;
            fn apply(self, b: &mut B, _world: &World) {
                self(b, $(_world.resource::<$P>(),)*);
            }
        }
    }
}

all_tuples!(impl_class_tuple, 0, 5, P);

macro_rules! impl_class_more_tuple {
    ($(($P: ident, $p: ident)),*) => {
        #[allow(non_snake_case)]
        impl<B, $($P, $p),*> Class<(B, $($P,)*)> for ($($p,)*)
        where
            $($p: Class<$P, In = B>,)*
        {
            type In = B;
            fn apply(self, _b: &mut Self::In, _world: &World) {
                let ($($p,)*) = self;
                $($p.apply(_b, _world);)*
            }
        }
    };
}

all_tuples!(impl_class_more_tuple, 0, 5, P, S);

pub trait EntityWriter {
    fn set(self, entity: &mut Option<Entity>);
    fn push(self, destination: &mut Vec<Entity>);
}

impl EntityWriter for Entity {
    /// Copies this entity into an Option.
    fn set(self, entity: &mut Option<Entity>) {
        *entity = Some(self);
    }
    /// Pushes a copy of this Entity into a Vec.
    fn push(self, entities: &mut Vec<Entity>) {
        entities.push(self);
    }
}
