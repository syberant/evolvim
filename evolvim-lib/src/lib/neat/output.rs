#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutputType {
    MouthHue,
    Eating,
    Turning,
    Accelerating,
    Fight,
}

impl OutputType {
    pub fn use_output<B: 'static>(
        &self,
        value: f64,
        env: &mut crate::brain::EnvironmentMut<B>,
        time_step: f64,
    ) {
        use OutputType::*;
        use crate::ecs_board::BoardPreciseCoordinate;

        match self {
            MouthHue => env.this_body.set_mouth_hue(value),
            Eating => {
                let rg_body = env.world.rigid_body_mut(env.handle).unwrap();

                let pos = rg_body.position().translation.vector;
                let tile_pos = BoardPreciseCoordinate(pos[0], pos[1]).into();

                let tile = env.terrain.get_tile_at_mut(tile_pos);
                env.this_body
                    .eat(value, time_step, env.time, env.climate, tile, rg_body);
            }
            Turning => {
                let rg_body = env.world.rigid_body_mut(env.handle).unwrap();
                
                env.this_body.turn(value, time_step, rg_body);
            },
            Accelerating => {
                let rg_body = env.world.rigid_body_mut(env.handle).unwrap();

                env.this_body.accelerate(value, time_step, rg_body);
            },
            Fight => {
                // env.this_body.fight(
                //     value,
                //     env.time,
                //     time_step,
                //     env.sbip,
                //     env.world,
                //     env.self_pointer.clone(),
                // );
            }
        };
    }
}
