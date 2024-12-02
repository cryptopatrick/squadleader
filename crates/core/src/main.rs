//! _squadleader_ is a crate containing a type system and a rule engine that is
//! based on the rules of the classic boardgame with the same name.
//! Please note, there's no affiliation between this makers of this crate
//! and the publishers of the boardgame.
//!
//! Further, this crate is not supposed to be a game. However, it can probably
//! be used as a starting point, create one.
//! The purpose of this crate is to model the types and rules of the original game.
//! You can think of it as a type system and rule engine modeling the game of chess,
//! without being an actual chess game that can be played.
//!
//! References to the original 4th Edition Squad Leader rules are included
//! in the source, and expressed in the following manner:
//! `SL1.1` refers to paragraph 1.1 in the original 4th ed. rule book.


// "Courage, above all things, is the first quality of a warrior."
// –Carl von Clausewitz

////////////////////////////////////////////////////////////////////////////////
/// SL1.1
/// A generic struct representing any target (building, unit, armour, etc).
/// Every target has a location (in terms of longitude, latitude), and an elevation.
/// Every target also has a armor value - a value that has to be super seeded for
/// any destructive force to have a chance to inflict damage on the target.
/// Finally, every target has a health value - how much damage it can withstand
/// before it is destroyed.
struct Target {
    location: HexId,
    elevation: u8,
    armor: u8,
    health: u8,
}

// A generic type describing anything on the battlefield that can be given an order.
// Anything that can be given an order can refuse to do so - usually due to being
// in a morally broken condition or having surrendered.
// For any type to have the Orderable trait they must implement the fn communicate.
pub trait Orderable {
    fn copythat() -> String;
}
// TODO: write a generic function that can be used to identify units that can communicate.


/// Units

/// Squad
/// SL2.2 Firepower: the basic strength that a unit or support weapon can attack with.
/// SL2.3 Range: the distance in game hexes, from the firing unit to the target, that
/// unit can reach with its normal firepower factor.
/// SL2.4 Morale: a rating of a unit's ability to withstand combat stress before
/// 'breaking down psychologically and fleeing'. A broken unit is non-operational
/// until receiving treatment by non-broken personel.
struct Squad {
    firepower: u8,
    range: u8,
    morale: u8,
//  condition: Condition,
}

/// Leader
/// SL2.5 Identity: the name and rank of the leader unit.
/// SL2.6 Leadership: a rating of a leader's ability of get the best performance
/// out of the combat units under his command.
/// The leadership number is added, as a modifier, to any morale or firepower
/// tests performed by a unit under the leaders comand.
/// Competence: a rating of a leader's tactical competence.
/// Morale: a rating of the leader's ability to withstand combat stress before
/// 'breaking down psychologically and fleeing'. A broken leader is non-operational
/// until receiving treatment by non-broken personel.
//
struct Leader {
    identity: String,
    leadership: u8,
    morale: u8,
    // todo: competence: u8,
    // condition: Condition,
}

impl Leader {
    // SL2.6 Leadership affects unit performance
    // TODO: incorporate modifier when calculating unit to hit or save morale.
    fn get_leadership_modifier(&self) -> u8 {
        return self.is_being_led().get_modifier()
    }
}

// TODO: move to morale
// A broken unit is psychologically broken down and unable to follow orders.
// The units survivial instincts will trumph any army discipline until the unit is rallied.
// A unit that is not broken is fully able to operate under combat conditions.
// TODO: improve naming of enum - perhaps Condition or Spirit - perhaps add Surrendered.
enum Condition {
    Broken,
    Composed,
}


/// Support Weapon
/// All suppport weapons must be operated by combat units.
/// Enemy support weapons can be captured and used by either side.
/// SL2.7 _Weapon_: A short description of the weapon type.
/// SL2.8 Penetration: a support weapon's ability to affect targets that are in
/// line with the target hex. A machine gun with a penetration value of 3
/// will have equal effect on the target hex, but also on the additional 2 hexes
/// that are in a straight line, along the line of sight, behind the target hex.
/// A penetration value of 1 will only affect the targeted hex.
/// SL2.9 _Breakdown_: combat environments are harsh. To express this, support weapons
/// have a number which determines if the weapon temporarily breaks down and malfunctions
/// during operation.
struct SupportWeapon {
    weapon: WeaponType,
    firepower: u8,
    penetration: u8,
    range: u8,
    breakdown: u8,
}

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

/// Vehicle
/// All vehicles must be operated by crews.
/// Enemy vechicles cannot be captured and used.
/// 18.1
struct Vechicle {
    mf: u8
}




////////////////////////////////////////////////////////////////////////////////
/// 3. Maps
/// Each hex represents a distance of 40 meters, a scale that was selected with
/// the hopes that it captures the flow of tactical small unit combat.


// Every hex has a unique id which can be referenced against a map database.
// The map database contains information about what's in the hex, including
// elevation data.
struct HexId(u32);

/// Returns the distance from hex A, to hex B in the number of inclusive hexes.
fn get_distance(origin: HexId, target: HexId) -> u8 {
    todo!()
}

/// Returns a boolean indicating if the unit located in origin hex can see
/// the enemy located in the target hex.
/// The requires a database hit due to the fact that all line of sight from
/// to any other hex on the map has been precalculated in to a lookup table.
/// Each hex has a center which is used to calculate Line of Sight (LOS) to other
/// hexes, for targeting purposes. A unit (except for mortar units) can only fire
/// on targets that they can see (that is in their LOS).
fn get_los(origin: HexId, target: HexId) -> bool {
    todo!()
}

/// Terrain will affect how fast units can move.
/// The Terrain Effect Function (TEF) calculates how a particular type of terrain
/// affects movement. A hex can contain multiple terrain types.
/// The cost of moving through such a hex is cumulative.
fn get_terrain_effect_on_move(hex: HexId) -> u8 {
    // types of terrain in the hex

    // SL3.8 accumulate terrain types to calculate effect
    todo!();
}

/// SL3.5 Each hex has a unique identifier which commanders can use to communicate
/// unit movement and targeting.
fn get_grid_coordinate() -> HexId {
    // Normally used when clicking on a map and reading the HexId of the hex.
    todo!()
}





////////////////////////////////////////////////////////////////////////////////
/// 4. Sequence of Play
/// Reaction is slower than action, therefore it is important to distinguish
/// between the first moving player and the second moving player.
/// A player can either attack or defend, and just like two boxers, a first mover
/// may want to preempt an attack by moving into a defensive position - an action
/// that would improve his defensive posture.
enum Player {
    Attacker,
    Defender,
}

/// A game turn consists of two complete eight phase player turns.
/// Each game turn represents 2 minutes of actual time.
/// The player who moves first is refered to as the Player::Attacker, while
/// the other player is refered to as Player::Defender.
enum Phase {
    // SL4.1
    // Both players can attempt to repair malfunctioning support weapons and
    // attempt to rally broken troops.
    Rally,
    // SL4.2
    // The Player::Attacker may order any of his units to fire on any
    // enemy units that are within the unit's line of sight.
    // A unit that fire under this phase is indicated graphically by a
    // Marker::PrepFire.
    PrepFire,
    // SL4.3
    // Player::Attacker may order unbroken units that did not fire during the
    // Phase::PrepFire to move.
    Movement,
    // SL4.4
    // Player::Defender may order any unbroken units to fire at any enemy units
    // that are either: 1) in their current LOS, or, 2) moved through their LOS
    // during Phase::Movement.
    DefensiveFire,
    // SL4.5
    // Player::Attacking may now order any of his units who moved during
    // Phase::Movement to fire at enemy units. The penalty for firing after
    // movement is that firepower is halfed, rounded up.
    // Units that did neither Phase::PrepFire nor Phase::Movement can fire at their
    // full firepower.
    // At the end of the AdvancingFire phase all Marker::PreFire are removed.
    // TODO: function for removing Marker::Prepfire.
    AdvancingFire,
    // SL4.6
    // Both players move their broken units into cover of terrain type:
    // Terrain::Wood, Terrain::Building. A unit
    Rout,
    // SL4.7
    // As a final push, Player::Attacker may now move any of his non-broken units
    // one hex forward. The hex moved into is allowed to contain enemy units.
    // This is the only phase in which combat troops are allowed to be moved into
    // a hex occupied by enemy units.
    Advance,
    // SL4.8
    // All units, on both sides, who find themselves in the same hex must
    // attack each other. Results are calculated using the Close Combat Table (CCT).
    CloseCombat,

    // After Phase:CloseCombat the Player::Attacker and Player::Defender switches.
    // Once both players have each reached the PhaseCloseCombat phase
    // then the current game turn is over, and the next one starts, or the game is over.
}

// Game Loop

fn update_scenario_turn() {
    // Every time Player::Attacker
    // Increment scenario game turn by one.
    // Check if scenario game turn has reached limit and the game is over.
    todo!()
}



////////////////////////////////////////////////////////////////////////////////
// SL05. Movement Orders

trait Orderable {
    fn copythat() -> String;
}
// Handles: Phase::Advance
// Any unit that the player selects to move must pass the checks of not being
fn advance_unit(unit: Orderable, dest: HexId ) -> bool {
    // if unit.condition != Condition::Broken {}
    todo!()
}

// A broken unit in cover has to move if it finds itself adjacent to an enemy unit.
// Handles: Phase::Rout
fn broken_in_cover() {
    // For every broken unit, check that no enemy unit is in an adjacent hex.
    // If an enemy is adjacent then the unit has to be routed into another cover.
    todo!("check that the unit is in a hex that is deemed of being TerrainType::Cover");
    todo!("check that the unit is not adjacent to enemy unit.")
}

////////////////////////////////////////////////////////////////////////////////
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
    if unit.moved {
        String::from("The unit moved and thus cannot fire during the advanced fire phase")
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
fn get_enpassant_targets() -> Vec<HexId> {
    // For each unit that moved during the movement phase
    // calculate if it passed through any enemy unit's LOS.
    // If a unit passed through the LOS of an enemy unit, a Marker::EnPassant
    // is added to the enemy unit in question.
    todo!("Calculate En passant targets and add them to list of viable targets.")
}

////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////
/// Orders
fn order_move(unit: HexId) {
    // Conditions to move
    // 1. The unit is not broken.
    // 2. The unit did not fire during Phase::PrepFire.


    // is_broken(unit.id)
    // has_prep_fired(unit.id)
    if unit.broken() {
        println!("{}", response_to_order(OrderResponse::UnitBroken));
    } else if unit.prepfired() {
        println!("{}", response_to_order(OrderResponse::PrepFired));
    } else {
        println!("{}", response_to_order(OrderResponse::CopyThat));
        unit.move_to_dest()

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



////////////////////////////////////////////////////////////////////////////////
/// Markers are used to indicate state and help the player get a sense of what's
/// going on in the battlefield.
enum Marker {
    PrepFire,   // The unit fired during the PrepFire phase.
    EnPassant,  // During its movement phase, the unit passed through one or more enemy unit's LOS .
    ProximityPanic,   // The marker indicates that a broken unit has to be moved because of enemy proximity.
}


// Fire Phase
/// Firing into or through a hex with multiple terrain types has a cumulative
/// effect.
fn terrain_effect_movement(hex: HexId) -> u8 {}




////////////////////////////////////////////////////////////////////////////////
// Game Loop
fn game_loop() {
    todo!("Implement a state machine which steps throug the game phases.");
}


////////////////////////////////////////////////////////////////////////////////
// Fire
/// The penetration value is always adjusted to 1 when firing from one elevation
/// to a different elevation.
/// Returns the adjusted penetration value of the unit.
fn sw_calc_penetration(origin: &HexId, target: &HexId) -> Option<u8> {
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

fn terrain_effect_combat(hex: HexId) -> u8 {
    // types of terrain in the hex
    // accumulate terrain types to calculate effect
}


////////////////////////////////////////////////////////////////////////////////
// Close Combat
// Handles: Phase::CloseCombat
fn close_combat() {
    // For all opposing units in the same hex
    // use the Close Combat Table to calculate results
    // Update state (remove casulties).
}


////////////////////////////////////////////////////////////////////////////////
// 5. Movement
// Each hex  in the game represents 50 meters (from border to border).
// SL5.1 Only units that did not move during the Phase::PrepFire phase may move during
// the Phase::Movement.
// TODO: get can't move because prep fired working.
/* fn unit_can_move(unit: Orderable) -> Option<bool, Feedback> {
    if !GameState.PrepFire.contains(unit) {
        Some(true,())
    } else {
        Some(false, Feedback::UNIT_PREP_FIRED)
    }
} */

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
fn calcuate_mf_bonus(s:Squad) -> u8 {
    if s.stacked_with_leader() {
        2
    } else {
        0
    }
}

// SL5.5 Moving into a hex has a MF cost, depending on the type of terrain.
// SL5.51 Crossing walls and hedges places a 1 MF penalty on the units MF.
// SL5.52 A unit moving from one road hex to another will only pay 1 MF for every second
// road hex it traverses.
// TODO: read up on enums and data types.
enum TerrainCost {
    OpenGround = 1,
    Shellhole = 1,
    Wheatfield = 1,
    OnRoad = 1,
    OntoRoad = 1,
    Woods = 2,
    EnterBuilding = 2,
    WithinBuilding = 2,
    OverWall=1,
}

// SL5.53 A Squad moving upwards, from TerrainCost::OpenGround, OnRoad, Building,
// and Woods, from one terrain level to a higher one, will double its MF cost.
// Moving along same level carries no bonus or additional cost.
// Nor is there a bonus for moving from one level to a lower level.
// TODO: add functionality to double terrain cost due to higher ground.
fn tfx_higher_ground<S:Squad>(orig: HexId, dest: HexId) -> bool {
    dest.elevation > orig.elevation
}

// SL5.54 Terrain effects are cummulative: hexes containig more than one terrain
// type have the MF of each type added together.
// TODO: add fn to cummulate terrain fx.

// SL5.6 Infantry units can move up to and around a hex containing enemy units
// but may only move into a hex containing an enemy unit during Phase:Advance.
// TODO: add fn which prohibts movements into enemy controlled hex.

// SL5.70-Carrying support weapons and portage costs.



////////////////////////////////////////////////////////////////////////////////
/// 7. Line of Sight (LOS)
// SL7.1 Line of sight - what a unit can "see" and fire upon, is measured from
// the center dot of the hex (of the unit) and a straight line to the center dot
// of the target hex.
// If an obstacle on the map can be observed on both sides of this line then the
// LOS is obstructed.
fn los_obstructed(orig: HexId, dest: HexId) -> bool {
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


////////////////////////////////////////////////////////////////////////////////
// SL8
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// SL16 Defensive Fire Principles

////////////////////////////////////////////////////////////////////////////////
// SL16 Defensive Fire Principles

////////////////////////////////////////////////////////////////////////////////
// SL15 Leadership

////////////////////////////////////////////////////////////////////////////////
// SL16 Defensive Fire Principles

////////////////////////////////////////////////////////////////////////////////
// SL16 Defensive Fire Principles

////////////////////////////////////////////////////////////////////////////////
// SL17 Machine Guns

////////////////////////////////////////////////////////////////////////////////
// SL18 Fate

////////////////////////////////////////////////////////////////////////////////
// SL19 Movement and Fire

////////////////////////////////////////////////////////////////////////////////
// SL20 Close Combat
