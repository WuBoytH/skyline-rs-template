#[allow(unused_imports)]
use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    smash_script::*
};

// mario
mod acmd;
mod status;
mod frame;

// mario_fireball
mod fireball;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    fireball::install();
}
