/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */
use bevy::prelude::*;
use colored::Colorize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    App::new()
        .add_startup_system(add_npcs)
        .add_startup_system(add_players.after(add_npcs))
        .add_system(list_npcs)
        .add_system(list_players.after(list_npcs))
        .run();
}

// Characters
#[derive(Component)]
struct Npc;

#[derive(Component)]
struct Player;

// Player - data
#[derive(Component)]
struct PlayerIpAddr(IpAddr);

// Characters - data
#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Health(u8);

#[derive(Component)]
struct Killable(bool);

fn add_npcs(mut commands: Commands) {
    commands
        .spawn()
        .insert(Npc)
        .insert(Name("Fisherman".to_string()))
        .insert(Health(100))
        .insert(Killable(true));
}

fn list_npcs(query: Query<(&Name, &Health, &Killable), With<Npc>>) {
    println!("{}", "NPCs".bold());
    let iter = query.iter();
    for (i, (name, health, killable)) in &mut iter.enumerate() {
        println!("\tID {}:", &i);
        println!("\t\tName:\t\t{}", name.0);
        println!("\t\tHealth:\t\t{}", health.0);
        println!("\t\tKillable:\t{}", killable.0);
    }
}

fn add_players(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Name("player-0".to_string()))
        .insert(Health(100))
        .insert(Killable(true))
        .insert(PlayerIpAddr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
    commands
        .spawn()
        .insert(Player)
        .insert(Name("player-1".to_string()))
        .insert(Health(100))
        .insert(Killable(true))
        .insert(PlayerIpAddr(IpAddr::V6(Ipv6Addr::new(
            0, 0, 0, 0, 0, 0, 0, 1,
        ))));
}

fn list_players(query: Query<(&Name, &Health, &Killable, &PlayerIpAddr), With<Player>>) {
    println!("{}", "PLAYERS".bold());
    let iter = query.iter();
    for (i, (name, health, killable, player_ip_addr)) in &mut iter.enumerate() {
        println!("\tID {}:", &i);
        println!("\t\tName:\t\t{}", name.0);
        println!("\t\tHealth:\t\t{}", health.0);
        println!("\t\tKillable:\t{}", killable.0);
        println!("\t\tIP:\t\t{}", player_ip_addr.0);
    }
}
