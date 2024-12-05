#![allow(unused)]
//! _squadleader_ is a crate containing a type system and a rule engine that is
//! based on the rules of the classic boardgame with the same name.
//! Please note, there's no affiliation between this makers of this crate
//! and the publishers of the boardgame.
//!
//! Further, this crate is not supposed to be a game. However, it can probably
//! be used as a starting point, create one.
//! The purpose of this crate is to model types and rules of the original game.
//! You can think of it as a type system and rule engine modeling the game
//! of chess, without being an actual playable chess game.
//!
//! References to the original 4th Edition Squad Leader rules are made
//! in the source, and expressed in the following manner:
//! `SLX.Y` refers to paragraph X.Y in the original 4th ed. rule book.
//! All rules, that do not reference the original game, are labled RX.Y.

// "Courage, above all things, is the first quality of a warrior."
// –Carl von Clausewitz

////////////////////////////////////////////////////////////////////////////////
// Place all imports below:
use std::collections::HashMap;
use std::marker;

////////////////////////////////////////////////////////////////////////////////
/// SL1. Addmendums, labeled AX.Y, made to the original SL rule set.
///
/// A1.1
/// A generic struct representing any target (building, unit, armour, etc).
/// Every target has a location (in terms of longitude, latitude), and an
/// elevation. Every target also has an armor value - a value that has to be
/// super seeded for any destructive force to have a chance to inflict damage
/// on the target. Finally, every target has a health value - how much damage it
/// can withstand before it is destroyed.
struct Target {
    location: Hex,
    elevation: u8,
    armor: u8,
    health: u8,
}

/// A struct reponsible for tracking relevant events on the battlefield.
struct BattleManager {}

/// The ScenarioManager is reponsible for tracking scenrio relevant events on
/// the battlefield. TODO: give examples of scenerio events.
struct ScenarioManager {}

/// Marker States
// A1.2
// The state of a non-broken combat unit at the start of a new Game Turn.
struct Unphased;
// SL5.1
// Any combat unit that moved during Phase::PrepFire of the current GameTurn.
// TODO: Decide on naming of states: struct PrepFireMoved;
struct PrepFired;
// Asserts SL5.75
// TODO: consider other solutions here.
// TODO: what types of support weapons are there?
struct FiredSupportWeapon;

////////////////////////////////////////////////////////////////////////////////
/// SL2. Combat Units
///
/// SL2.1 Unit: a unit represents any infantry squad, officer, or NCO
/// (Non-Commissioned Officer), present on the battlefield.
//  TODO: consider weather to treat Armour crews and AT Gun crews as units.
/// Unit is a Generic Type representing any unit (leader, squad, armour crew,
/// Anti Tank Gun crew) on the battlefield.
#[derive(Debug)]
enum Unit {
    // TODO: _consider: Are Armour and ATGun treated differently in terms of
    // PrepFire and limitations to Movement?
    Armour,
    ATGun,
    Leader,
    Squad,
}

impl Unit {
    fn has_moved(&self) -> bool {
        todo!()
    }

    fn has_prepfired(&self) -> bool {
        todo!()
    }

    fn broken(&self) -> bool {
        todo!()
    }

    fn move_to_dest(&self) -> bool {
        todo!()
    }
}

/// SL2.2 Firepower: the basic strength that a unit can attack with in combat.
/// SL2.3 Range: the distance, calculated as the least number of game hexes,
/// from the firing unit to the target, that a unit can reach with its normal
/// firepower factor.
/// SL2.4 Morale: a rating of a unit's ability to withstand combat stress before
/// 'breaking down psychologically and fleeing'. A broken unit will remain
/// broken until it has received successful treatment by non-broken personel.
use std::net::Ipv4Addr;
#[derive(Debug)]
struct Squad<State> {
    // The `ars` is an identifying value, based on the United States Army
    // Regimental System (USARS) - an organizational and classification system
    // used by the United States Army. The `ars` provides each squad and leader
    // on the battlefield with a unique identification, within a single
    // regiment. A squad consists of 6-10 soldiers. To simplify
    // identification on the battlefield, we have decided to treat the whole
    // squad as one single entity (solider), with its unique ars.
    //
    // Let's say that we want to model the following army:
    //
    // + 1 Battalion: 100-1_000 soldiers, comprised of 3-5 companies
    // + 3 Companies: each with 60-200 soldiers, comprised of 3-4 platoons
    // + Platoon: 18-50 soldiers, comprised of 3-4 squads
    // + Squad: comprised of 6-10 soldiers
    //
    // Example
    // ```
    // use std::net::Ipv4Addr;
    //
    // let squad_id = Ipv4Addr::new(Battalion, Company, Platoon, Squad);
    // //1st Battalion, 3rd Company, 4th Platoon, 4th Squad becomes:
    // let squad_id = Ipv4Addr::new(1, 3, 4, 4);
    // ```
    //
    pub ars: Ipv4Addr,
    firepower: u8,
    range: u8,
    morale: u8,
    // Marker Type used to model illegal actions based on state.
    // See for example SL5.1.
    _state: marker::PhantomData<State>, // TODO: A way to model that a unit is either broken or non-broken.
                                        // condition: Condition,
}

impl Squad<Unphased> {
    fn has_moved(&self) -> bool {
        todo!()
    }
    fn stacked_with_leader(&self) -> bool {
        todo!()
    }
}

// SL2.6 Leadership affects unit performance
impl Squad<Unphased> {
    // TODO: modifier when calculating `unit To Hit` or `unit save morale`.
    fn get_leadership_modifier(&self) -> u8 {
        todo!()
        /*
        match self.led_by()? {
            None => 0,
            Some => self.led_by().leadership(),
        }
        */
    }
}

/// SL2.5 Identity: the name and rank of the leader unit.
/// SL2.6 Leadership: a rating of a leader's ability to get the best performance
/// out of the combat units under his command.
/// The leadership number, usually negative, is added as a modifier to any
/// morale or firepower test performed by a unit under the leader's command.
///
struct Leader {
    identity: String,
    pub leadership: u8,
    // Morale: a rating of the leader's ability to withstand combat stress
    // before breaking down psychologically, and fleeing'.
    // A broken leader is non-operational until receiving treatment by
    // non-broken personel.
    //
    // TODO: Using illegal states, make sure that leadership modifier to units
    // is only applicable if the leader himself is non-broken.
    morale: u8,
    // TODO: _WIP Ideas for fields to add:
    // Competence: a rating of a leader's tactical competence.
    //
    // TODO: A way to model that a leader is either broken or non-broken.
    // condition: Condition,
}

// TODO: _code-structure: move to morale
// TODO: perhaps improve naming of enum?
// TODO: perhaps add Surrendered?
/// A non-broken unit is fully able to operate under combat conditions.
/// A broken unit is psychologically broken down and unable to follow orders.
/// The unit's survivial instincts will trumph any army discipline until the
/// unit is rallied into a non-broken state.
///
enum Condition {
    Broken,
    Composed,
}

/// SL2.7 Weapon Type: A short description of the weapon type.
/// All suppport weapons must be operated by combat units.
/// Enemy support weapons can be captured and used by either side.
/// SL2.8 Penetration: a support weapon's ability to affect targets that are in
/// line with the target's `Hex`. A machine gun with a penetration value of 3
/// will have equal effect on the target hex, but also on the additional 2 hexes
/// that are in a straight line, along the line of sight, behind the target hex.
/// A penetration value of 1 will only affect the targeted hex.
/// SL2.9 _Breakdown_: combat environments are harsh.
/// To express this, support weapons have a number which determines if the
/// weapon temporarily breaks down and malfunctions during operation.
///
enum WeaponType {
    Rifle,
    LMG,
    MMG,
    HMG,
    Mortar,
    Demolition,
    Flamethrower,
    APMine,
    ATMine,
    ATRifle,
    ATGun,
    TankGun,
    Artillery,
}

struct SupportWeapon {
    weapon: WeaponType,
    firepower: u8,
    penetration: u8,
    range: u8,
    breakdown: u8,
}

/// SL18.1
/// All vehicles must be operated by crews.
/// Enemy vechicles cannot be captured and used.
struct Vechicle {
    mf: u8,
}

////////////////////////////////////////////////////////////////////////////////
/// SL3. Maps
///
/// Each hex represents a distance of meters on the battlefield.
/// The default hexdistance is 40 meters, but that value can be changed.
/// The distance must be chosen with care and with intetion to capture the flow
/// of tactical small unit infantry combat.
///
// TODO: add functionality to change the HEXDISTANCE.
static HEXDISTANCE: AtomicU8 = AtomicU8::new(40);

/// SL3.2 A hexagon shaped grid is superimposed on the battlefield map.
/// The default is for the hexagon grid to be six-sided and visible.
///
// TODO: Add functionality to toogle the visibility of the hexagon.
// const mut CONST DRAWHEXAGONS:bool = true;
///
/// Every hex has a unique id which can be referenced against a map database.
/// The map database contains information about what's in the hex, including
/// elevation data, and the category of terrain that the hex belongs to.
// TODO: Add map database connectivity functionlality.
///
/// SL3.6 Map designers can include aesthetic elements which have no terrain
/// effect on movement. It's important to note that each hex on a map __can__
/// belong to more than one terrain category. The effect of this on movement
/// and combat resolution is __cumulative__.
/// For example, a hex can contain both a TERRAIN::HEDGES and TERRAIN::WOODS.
/// The cost of moving through that hex would be calculated by adding the cost
/// of both terrain types.
///
//  TODO: Implement terrain category in database.
struct Hex {
    id: u32,
    elevation: u8,
    // TODO: not super happy of type-setting terrain as a Vec<Terrain>, but how
    // else can we handle the fact that a single hex could contain
    // a house on a hill surrounded by a hedge?
    terrain: Vec<Terrain>,
}

impl Hex {
    fn broken(&self) -> bool {
        todo!()
    }
}

/// Returns the distance from hex A, to hex B in the number of inclusive hexes.
fn get_distance(origin: Hex, target: Hex) -> u8 {
    todo!()
}

/// Returns a boolean indicating if the unit located in origin hex can see
/// the enemy located in the target hex.
/// The requires a database hit due to the fact that all line of sight from
/// to any other hex on the map has been precalculated in to a lookup table.
/// Each hex has a center which is used to calculate Line of Sight (LOS) to other
/// hexes, for targeting purposes. A unit (except for mortar units) can only fire
/// on targets that they can see (that is in their LOS).
fn get_los(origin: Hex, target: Hex) -> bool {
    todo!()
}

/// SL3.1 Terrain will affect how fast units can move through a hex on the map.
/// The Terrain Effects Chart (or TEC) is a database containing a description of
/// all terrain types.
//  TODO: add TEC database and connectivity.
///
/// The Terrain Effect Function (TEF) calculates how a particular type of terrain
/// affects movement. A hex can contain multiple terrain types.
/// The cost of moving through such a hex is cumulative.
fn get_terrain_effect_on_move(hex: Hex) -> u8 {
    // types of terrain in the hex
    // SL3.8 accumulate terrain types to calculate effect
    todo!();
}
/// SL3.3 Each hex on the map has a center point. This point is used to
/// calculating line of sight (LOS) between two different hexes on a map.
/// These points are important because combat units are assumed to be firing
/// from these center points towards center points in other hexes.

/// The LOS calculations are persisted in the each map database and then
/// used during simulation to determine if there's a clear LOS between two hexes.
//  TODO: write a function that can, if provided with a map data file, generate
//  LOS between two hexes on that map and persist it.

/// SL3.4 Map designers should aim for an `isomorphic` quality in their maps
/// making it possible to combine the edge of each map to any other map.
//  TODO: write functionality to determine if two maps can be combined, edge to
//  edge, and flag if the maps are in fact incompatible.
/// SL3.7 The half hexes along the edge of the a map are all treated as full
/// hexes in terms of terrain and line of sight.
//  TODO: EDGE CASE: edge of maps contain half hexes to enable isomorphic
//  combination of maps. Write functionality to handle this edge case.

/// SL3.5 Each hex has a unique identifier which commanders can use to communicate
/// unit movement and targeting.
//  TODO: consider how Hex of edge hexes is determined for combined maps.
//  The likely solution is to have the halves of each edge hex to create a new
//  hex, however the id of that hex is something that needs to be considered.
/// The classic Hex format is:
/// 1AB3 where <MAPID(1A)-ROWLETTER(B)-ROWHEXNUMBER(3)>
fn get_grid_coordinate() -> Hex {
    // Normally used when clicking on a map and reading the Hex of the hex.
    todo!()
}

////////////////////////////////////////////////////////////////////////////////
/// SL4. Sequence of Play
///
/// A game turn consists of two complete eight phase player turns. Meaning, each
/// player will perform the 8 phases, for a combined total of 16 phases.
/// The 16 phases represent one complete game turn.
/// Each game turn represents 2 minutes of actual time.
use std::sync::atomic::{AtomicU8, Ordering};
// TODO: add functionality to change the length of a GAMETURN.
static GAMETURN: AtomicU8 = AtomicU8::new(2);

/// Reaction is slower than action, therefore it is important to distinguish
/// between the first moving player and the second moving player.
/// A player can either attack or defend, and just like two boxers, a first
/// mover may want to preempt an attack by moving into a defensive position -
/// an action that would improve his defensive posture.
/// In each game turn, one player will be the defender, while the othe will be
/// the attacker, they then switch and the defender becomes the attacker.
/// It should be noted that it's the attacker who goes through the phases.
enum Player {
    Attacker,
    Defender,
}

/// The player who moves first in a game turn is refered to as the
/// Player::Attacker, while the other player is refered to as Player::Defender.
enum Phase {
    // SL4.1
    // Both players can attempt to repair malfunctioning support weapons and
    // attempt to rally broken units.
    // TODO: add fn repair_malfunctioning_support_weapon()
    // TODO: add fn rally_broken_unit().
    Rally,
    // SL4.2
    // The Player::Attacker may order any of his units to fire on any
    // enemy units that are within the unit's line of sight.
    // TODO: Any unit that fires under this phase is indicated graphically by a
    // Marker::PrepFire.
    PrepFire,
    // SL4.3
    // Player::Attacker may order unbroken units that did not Phase::PrepFire
    // to execute a movement order.
    Movement,
    // SL4.4
    // Player::Defender may order any unbroken units to fire at any enemy units
    // that are either: 1) in their current LOS, or, 2) moved through their LOS
    // during Phase::Movement.
    DefensiveFire,
    // SL4.5
    // Player::Attacker may now order any of his units who moved during
    // Phase::Movement to fire at enemy units. The penalty for firing after
    // movement is that firepower is halfed, rounded down.
    // TODO: assert that firepower is halved (rounded down) for units that moved.
    // Units that did neither Phase::PrepFire nor Phase::Movement can fire at their
    // full firepower.
    // At the end of the AdvancingFire phase all Marker::PreFire are removed.
    // TODO: function for removing Marker::Prepfire.
    AdvancingFire,
    // SL4.6
    // Both players, first Player::Attacker, then Player::Defender, __must __
    // move their broken units into cover of terrain type either
    // Terrain::Wood or Terrain::Building. A unit that is already in one of
    // of these two terrain types, need not move, __unless__, it is adjacent
    // to an enemy unit.
    // TODO: fn to check if a broken unit has to move despite being located in a
    // Wood hex or Building hex, due to being adjacent to an enemy unit.
    Rout,
    // SL4.7
    // As a final push, Player::Attacker may now move any of his non-broken units
    // one hex forward. The hex moved into is allowed to contain enemy units.
    // This is the only phase in which combat troops are allowed to be moved into
    // a hex occupied by enemy units.
    // TODO: add fn to prohibit all movement of opposing units into the
    // same hex, unless it's Player::Attacker doing so in Phase::Advance.
    // TODO: consider exception 27, 53.4, 56, 57.
    Advance,
    // SL4.8
    // All units, on both sides, who find themselves occupying the same hex must
    // attack each other. Results are calculated using the Close Combat Table (CCT).
    CloseCombat,
}

// SL4.9 Game Turn
// After the end of SL4.8 Phase:CloseCombat the Player::Attacker and
// Player::Defender switches.
// Once both players have each reached Phase::CloseCombat, then the current
// game turn is over, and the next one starts, or the game is over.
//
// A Game Turn is considered complete when both the attacking entity and the
// defending entity have gone through steps SL4.1 to SL4.8.
// TODO: The Battlemanger can now increment the gameturn_counter by 1.
// TODO: If  a scenario is being modeled then increment the Scenario Turn Record
// Counter by 1.
fn update_scenario_turn() {
    // Phase.counter() mod 16 represents the number of complete Game Turns.
    // Everytime the result is 0, increment the scenario game turn by one.
    // Check if scenario game turn has reached limit and the game is over.
    // TODO: implement Phase.counter() mod 16
    // TODO: implement check if Phase.counter() > Scenario Turn Record Chart
    // which would mean that the simulation/game is over.
    todo!()
}

////////////////////////////////////////////////////////////////////////////////
// SL5. Movement
//
// A trait describing anything on the battlefield that can be given an order.
// An `Orderable` type can refuse to execute the order - usually due to being
// in a morally broken condition, or having surrendered to enemy units.

// For any type to have the Orderable trait, must implement `fn communicate.`
// TODO: create a trait that can be used to identify units who can communicate.
pub trait Orderable {
    fn copythat() -> String;
}

/// A trait denotating any type that can carry equipment and portable weapons.
pub trait Carrier {
    fn carry() -> bool;
    fn abandon() -> bool;
}

// Handles: Phase::Advance
// Any unit that the player selects to move must pass the checks of not being
fn advance_unit<T: Orderable>(unit: &T, dest: Hex) -> bool {
    // if unit.condition != Condition::Broken {}
    todo!()
}

// A broken unit in cover has to move if it finds itself adjacent to an enemy unit.
// Handles: Phase::Rout
fn broken_in_cover() {
    // For every broken unit, check that no enemy unit is in an adjacent hex.
    // If an enemy is adjacent then the unit has to be routed into another cover.
    todo!("check unit is in a hex that is considered: TerrainType::Cover");
    // TODO: todo!("check that the unit is not adjacent to enemy unit.")
}

// Fire Orders
fn order_prep_fire() {
    todo!("place graphical Marker::PrepFire on units which fired during Phase::PrepFire")
}
fn order_defensive_fire() {
    todo!("Determine which units that are viable targets based on LOS")
}

fn order_moved_fire(unit: Unit, target: Unit) {
    // The unit's firepower is halfed because it moved during Phase::Movement
    // and thus did not have time to properly find targets and aim at them.
    // unit.firepower / 2
    todo!("Calculate units halved firepower, rounded up.")
}

fn order_aimed_fire(unit: Unit, target: Unit) {
    // The unit neither prep fired nor moved. This enable the unit to fire at
    // its full firepower.
    // Unit's which Phase::PrepFire may not fire during the advancing fire phase.
    if unit.has_moved() {
        String::from("The unit moved and thus cannot fire during the advanced fire phase");
    } else {
        determine_fire_effect(unit, target);
    }
    todo!("Fire with full firepower at enemy unit.")
}

fn determine_fire_effect(attacker: Unit, defender: Unit) {
    todo!()
}

// Player::Defender may order any unbroken units to fire at any enemy units
// that are either: 1) in their current LOS, or, 2) moved through their LOS
// during Phase::Movement. All targets that passed through the LOS are marked
// as Marker::EnPassant, after the chess rule.
fn get_enpassant_targets() -> Vec<Hex> {
    // For each unit that moved during the movement phase
    // calculate if it passed through any enemy unit's LOS.
    // If a unit passed through the LOS of an enemy unit, a Marker::EnPassant
    // is added to the enemy unit in question.
    todo!(
        "Calculate En passant targets and add them to list of viable targets."
    )
}

/// OrderResponse
/// Not every order by the commander can be executed, there can be many reasons why.
enum OrderResponse {
    CopyThat, // The unit has received the order and will attempt to execute it."
    UnitBroken,
    PrepFired,
}

fn response_to_order(response: OrderResponse) -> String {
    match response {
        OrderResponse::CopyThat => String::from("Copy that, Sir! Executing order."),
        OrderResponse::UnitBroken => String::from("The unit is broken and unable to execute the order."),
        OrderResponse::PrepFired => String::from("The unit fired during the prep fire phase and is therefore not eligable for this order."),
    }
}

/// Orders
fn order_move(unit: Unit) {
    // Conditions to move
    // 1. The unit is not broken.
    // 2. The unit did not fire during Phase::PrepFire.

    // is_broken(unit.id)
    // has_prep_fired(unit.id)
    if unit.broken() {
        println!("{}", response_to_order(OrderResponse::UnitBroken));
    } else if unit.has_prepfired() {
        println!("{}", response_to_order(OrderResponse::PrepFired));
    } else {
        println!("{}", response_to_order(OrderResponse::CopyThat));
        unit.move_to_dest();
    }

    // Instead of each game element keeping track of its own state, perhaps it's
    // better to have an hashmap tracking all broken units
    // and another hashmap tracking all units that prep fired.
    todo!("Use hashmap solution instead of struct property.")
}

// Move a unit from its current location to the provided destination.
// Terrain effects will affect if the unit can reach the destination.
fn move_to_dest() {
    todo!()
}

/// Markers are used to indicate state and help the player get a sense of what's
/// going on in the battlefield.
enum Marker {
    PrepFire,       // The unit fired during the PrepFire phase.
    EnPassant, // During its movement phase, the unit passed through one or more enemy unit's LOS .
    ProximityPanic, // The marker indicates that a broken unit has to be moved because of enemy proximity.
}

// Fire Phase
/// Firing into or through a hex with multiple terrain types has a cumulative
/// effect.
fn terrain_effect_movement(hex: Hex) -> u8 {
    todo!()
}

// Game Loop
fn game_loop() {
    todo!("Implement a state machine which steps throug the game phases.");
}

// Fire
/// The penetration value is always adjusted to 1 when firing from one elevation
/// to a different elevation.
/// Returns the adjusted penetration value of the unit.
fn sw_calc_penetration(origin: &Hex, target: &Hex) -> Option<u8> {
    if origin.elevation == target.elevation {
        Some(1)
    } else {
        None
    }
    // TODO: move this into the fire sequence of the support weapon?
}

/// Support weapons that can penetrate more than one hexes can select which hexes
/// to target, and these hexes need not be adjacent to each other.
/// For example: a light machine gun with a penetration of 3 can affect three
/// hexes in total, the target hex and two more hexes along it's line of sight.
/// In the following array, T is the target hex. The first and second P represent
/// hexes that are not adjacent to eacher, but they can still be penetrated.
/// However, there cannot be any obstacles (like buildings and woods) between
/// the penetration hexes.
/// -> [_,T,_,P,_,P,_,_]
fn can_penetrate() -> bool {
    todo!()
}

fn terrain_effect_combat(hex: Hex) -> u8 {
    // types of terrain in the hex
    // accumulate terrain types to calculate effect
    todo!()
}

// Close Combat
// Handles: Phase::CloseCombat
fn close_combat() {
    // For all opposing units in the same hex
    // use the Close Combat Table to calculate results
    // Update state (remove casulties).
}

////////////////////////////////////////////////////////////////////////////////
// SL5. Movement
//
// TODO: look to create a global const that can be set.
// TODO: check if this information is redundant.
// Each hex in the game represents 40 meters (from hex border to hex border).
//
// SL5.1 Only units that _did not fire_ during the Phase::PrepFire may move
// during the ensuing Phase::Movement.
// TODO: mechanism for limiting condition to _ensuing_.
// TODO: _consider using TypeState to assert rules.
impl BattleManager {
    fn unit_can_move(&self, u: &Unit, p: &Phase) {
        todo!()
        /*
        // Check the HashMap, did the unit move during the prep fire phase?
        match !self.Moved.contains_key(p, u) {
            Ok() => return true,
            Err(e) => return RuleBreak::E5100, //UNIT_TRYING_TO_MOVE_POST_PREP_FIRED
        }
        */
    }
}

// SL5.2 Units can move inside the limits of their Movement Factors (MF).
// The number of hexes that the unit can move is a function of its MF and
// various factors, such as leaders, terrain, presence of enemy units, items
// carried, enemy fire, etc.

// SL5.3.1 Unites can move over a hex containing other units.
// SL5.3.2 One hex can contain more than one unit but not more than ???
// TODO: decide how many units that can be stacked in one hex.

// SL5.4 All counters, except vehicles, have movement factors (MF) alloted to them:
// Support Weapons must be carried by Squad, Leader, or Vehicle
// TODO: rules to handle movement of support weapons by leader, squad, or vehicle.
enum MF {
    Squad = 4, // SL5.41
    Leader = 6, // SL5.42
               // Vehicles MF is a field of vechicle structs.
               // TODO: implement Vehicle struct with MF.
}

// SL5.44 If a squad spends the entire Phase::Movement in the company with
// a leader, then it will recive a MF bonus of 2.
// TODO: handle led by Leader and stacked with Leader for entire movement phase.
fn calcuate_mf_bonus(s: Squad<Unphased>) {
    todo!()
    /*
    if s.stacked_with_leader() {
        2
    } else {
        0
    }
    */
}

// SL5.5 Moving into a hex has a MF cost, depending on the type of terrain.
// SL5.51 Crossing walls and hedges places a 1 MF penalty on the units MF.
// SL5.52 A unit moving from one road hex to another will only pay 1 MF for every second
// road hex it traverses.
// TODO: read up on enums and data types.
enum Terrain {
    OpenGround,
    Shellhole,
    Wheatfield,
    OnRoad,
    OntoRoad,
    Woods,
    EnterBuilding,
    WithinBuilding,
    OverWall,
}

impl Terrain {
    //TODO: based on unit type or armour.
    //TODO: _consider: should these be pulled from db?
    fn movement_cost(&self) -> u32 {
        match self {
            Terrain::OpenGround => 1,
            Terrain::Shellhole => 1,
            Terrain::Wheatfield => 1,
            Terrain::OnRoad => 1,
            Terrain::OntoRoad => 1,
            Terrain::Woods => 2,
            Terrain::EnterBuilding => 2,
            Terrain::WithinBuilding => 2,
            Terrain::OverWall => 1,
        }
    }
}
// SL5.53 A Squad moving upwards, for example from TerrainCost::OpenGround;
// OnRoad, Building, and Woods, from one terrain level to a higher one, will
// double its MF cost.
//
// Moving along same level carries no bonus or additional cost.
// Nor is there a bonus for moving from one level to a lower level.
// TODO: add functionality to double terrain cost due to higher ground.
// TODO: verify the use of Type State below.
impl Squad<Unphased> {
    fn tfx_higher_ground(&self, orig: Hex, dest: Hex) -> bool {
        dest.elevation > orig.elevation
    }
}

// SL5.54 Terrain effects are cummulative: hexes containig more than one terrain
// type have the MF of each type added together.
// TODO: add fn to cummulate terrain fx.

// SL5.6 Infantry units can move up to and around a hex containing enemy units
// but may only move into a hex containing an enemy unit during Phase:Advance.
// TODO: add fn which prohibts movements into enemy controlled hex.

// SL5.70-Carrying support weapons and portage costs.
//

// SL5.73 Regardless of terrain and/or weapons portage, a squad or crew may
// always carry up to 5 (TODO: turn into global CONST?) portage points, and
// a leader carry 3 portage points, up to 1 hex during the `Phase::Advance`.
fn carry_during_advance_phase(c: impl Carrier, p: Phase) {
    todo!()
    /*
    if c::Squad && u.portage < 6 && p == Phase::Advance {
        // TODO: instruct BattleManager / SceneManger to move unit one hex.
    } else if u::Leader && u.portage < 4 && p::Advance {
        // TODO: instruct BattleManager / SceneManger to move unit one hex.
    } else {
        // TODO: Improve mechanism for Informing General.
        println!("Unit is too encoumbered to move.")
    }
    */
}

// SL5.74 A `Squad` carrying 4 or more portage points, or a `Leader` carrying 2
// or more portage points (TODO: turn into global CONST), during the
// `Phase::Movement` may _not_ fire a `SupportWeapon` during the _ensuing_
// `Phase::AdvancedFire`.
fn may_fire_support_weapon(u: Unit, s: SupportWeapon, p: Phase) {
    todo!()
    /*
    if p::AdvanceFire && (u::Leader.portage < 2 || u::Squad.portage < 4) {
        // May fire support weapon.
        // TODO: issue to handler responsible for the firing of weapons.
    } else {
        // TODO: add "Inform General" functionality to inform when commands are
        // break a rule. Also log to BattleManager?
        //
        println!("Unit {:?} may not fire the support weapon {:?} because the unit
        exceeded the allowed portage points during the movement phase {:?}.", u, s, p);
    }
    */
}

// Handled by A1.2.
// TODO: move this to fire section.
// SL5.75 An infantry unit may only fire one _type_ of `SupportWeapon` in the
// same Phase::FirePhase, Phase::PrepFire, or Phase::AdvanceFire.
// WeaponType::{LMG, MMG, HMG} are all considred weapons of the same _type_.
// TODO: enforce this how? Perhaps by each unit tracking which weapon type
// it has fired in the same fire phase:
//
// ```
// Unit.fire(w: WeaponType) {
//  if !Unit.weapon_types_fired.contains(w) {
//      w.fire();
//      Unit.weapon_types_fired.insert(w)
//  }
// }
// ```
// ...or let a battlemanager track and decide if a unit is allowed to fire
// a particular type of support weapon.

// SL5.76 Portage costs are assumed to refer to `Squad` usage costs.
// TODO: Consider how `Leader` should be handled with relation to portage cost.

////////////////////////////////////////////////////////////////////////////////
/// SL6. Stacking

////////////////////////////////////////////////////////////////////////////////
/// SL7. Line of Sight (LOS)
// SL7.1 Line of sight - what a unit can "see" and fire upon, is measured from
// the center dot of the hex (of the unit) and a straight line to the center dot
// of the target hex.
// If an obstacle on the map can be observed on both sides of this line then the
// LOS is obstructed.
fn los_obstructed(orig: Hex, dest: Hex) -> bool {
    // TODO: setup a database to hold all hex data, including LOS
    todo!("Look-table or database hit to get LOS for passed hexes.")
}

// SL7.2 LOS is only blocked if something in the hex is actually blocking it.
// It's not enough that there's woods or a build in the hex, the LOS actually
// has to be blocked.
// TODO: fn to verify LOS obstructed by gfx in hex.
// Weapon's fire may be traced through a hex containg units without affecting
// them, if the firer prefers this.
// TODO: option to fire through without affecting unit blocking LOs.
// TODO: Exception: 17.6

// SL7.3 LOS extends into woods and buildings but not through them.
// TODO: verify that LOS is obstructed by second woods/building hex so that
// fire is not able to pass through woods/buildings.

// SL7.4

///////////////////////////////////////////////////////////////////////////////
// SL8

///////////////////////////////////////////////////////////////////////////////
// SL16 Defensive Fire Principles

///////////////////////////////////////////////////////////////////////////////
// SL16 Defensive Fire Principles

///////////////////////////////////////////////////////////////////////////////
// SL15 Leadership

///////////////////////////////////////////////////////////////////////////////
// SL16 Defensive Fire Principles

///////////////////////////////////////////////////////////////////////////////
// SL16 Defensive Fire Principles

///////////////////////////////////////////////////////////////////////////////
// SL17 Machine Guns

///////////////////////////////////////////////////////////////////////////////
// SL18 Fate

///////////////////////////////////////////////////////////////////////////////
// SL19 Movement and Fire

///////////////////////////////////////////////////////////////////////////////
// SL20 Close Combat
//
