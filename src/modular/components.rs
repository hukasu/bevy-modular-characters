use bevy::{
    ecs::{component::Component, entity::Entity},
    scene::InstanceId,
};

pub trait ModularCharacter: Component {
    fn id_mut(&mut self) -> &mut usize;
    fn instance_id_mut(&mut self) -> &mut Option<InstanceId>;
    fn entities_mut(&mut self) -> &mut Vec<Entity>;
    fn id(&self) -> &usize;
    fn instance_id(&self) -> Option<&InstanceId>;
    fn entities(&self) -> &Vec<Entity>;
}

macro_rules! create_modular_segment {
    ($name:ident) => {
        paste::paste! {
            #[derive(Debug, Component)]
            pub struct [<ModularCharacter $name>] {
                pub id: usize,
                pub instance_id: Option<InstanceId>,
                pub entities: Vec<Entity>,
            }
            impl ModularCharacter for [<ModularCharacter $name>] {
                fn id_mut(&mut self) -> &mut usize {
                    &mut self.id
                }

                fn instance_id_mut(&mut self) -> &mut Option<InstanceId> {
                    &mut self.instance_id
                }

                fn entities_mut(&mut self) -> &mut Vec<Entity> {
                    &mut self.entities
                }

                fn id(&self) -> &usize {
                    &self.id
                }

                fn instance_id(&self) -> Option<&InstanceId> {
                    self.instance_id.as_ref()
                }

                fn entities(&self) -> &Vec<Entity> {
                    &self.entities
                }
            }
        }
    };
}

create_modular_segment!(Head);
create_modular_segment!(Body);
create_modular_segment!(Legs);
create_modular_segment!(Feet);
