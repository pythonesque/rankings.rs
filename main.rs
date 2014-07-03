#![feature(macro_rules)]

use std::collections::{HashMap,SmallIntMap};
use std::iter::range_inclusive;
use std::i32;

#[deriving(Show,Eq,PartialEq,Hash)]

struct Team {
  name : &'static str,
  year : i32,
}

type Location = &'static str;

struct TeamDatabase {
  teams: SmallIntMap<Team>,
  teamIds: HashMap<Team, uint>,
  franchises: SmallIntMap<uint>,
  franchiseTeams: SmallIntMap<uint>,
  locations: SmallIntMap<Location>,
}

macro_rules! teams(
  ( $( $( $min:expr to $max:expr in $l:expr, $e:expr );* )...* ... )
=> ( {
    let mut _i = 0u;
    let mut _j = 0u;
    let mut _teams = SmallIntMap::new();
    let mut _franchises = SmallIntMap::new();
    let mut _locations = SmallIntMap::new();
    $(
      {
        _i += 1;
        $( {
          let _min : i32 = $min;
          let _max : i32 = $max;
          for year in range_inclusive(_min, _max) {
            _j += 1;
            _teams.insert(_j, Team { name: $e, year: year });
            _franchises.insert(_j, _i);
            _locations.insert(_j, $l);
          }
        } );* ;
      }
    );*
    (_teams, _franchises, _locations)
  } )
)

impl TeamDatabase {
  fn make() -> TeamDatabase {
    let (teams, franchises, locations) = teams!(
      2010 to 2014 in "Boston", "Celtics"...
      2010 to 2014 in "New Jersey", "Nets"...
      2010 to 2014 in "New York", "Knicks"...
      2010 to 2014 in "Philadelphia", "76ers"...
      2010 to 2014 in "Toronto", "Raptors"...

      2010 to 2014 in "Chicago", "Bulls"...
      2010 to 2014 in "Cleveland", "Cavaliers"...
      2010 to 2014 in "Detroit", "Pistons"...
      2010 to 2014 in "Indiana", "Pacers"...
      2010 to 2014 in "Milwaukee", "Bucks"...

      2010 to 2014 in "Atlanta", "Hawks"...
      2010 to 2013 in "Charlotte", "Bobcats"; 2014 to 2014 in "Charlotte", "Hornets"...
      2010 to 2014 in "Miami", "Heat"...
      2010 to 2014 in "Orlando", "Magic"...
      2010 to 2014 in "Washington", "Wizards"...

      2010 to 2014 in "Dallas", "Mavericks"...
      2010 to 2014 in "Houston", "Rockets"...
      2010 to 2014 in "Memphis", "Grizzlies"...
      2010 to 2012 in "New Orleans", "Hornets"; 2013 to 2014 in "New Orleans", "Pelicans"...
      2010 to 2014 in "San Antonio", "Spurs"...

      2010 to 2014 in "Denver", "Nuggets"...
      2010 to 2014 in "Minnesota", "Timberwolves"...
      2010 to 2014 in "Oklahoma City", "Thunder"...
      2010 to 2014 in "Portland", "Trail Blazers"...
      2010 to 2014 in "Utah", "Jazz"...

      2010 to 2014 in "Golden State", "Warriors"...
      2010 to 2014 in "Los Angeles" , "Clippers"...
      2010 to 2014 in "Los Angeles", "Lakers"...
      2010 to 2014 in "Phoenix", "Suns"...
      2010 to 2014 in "Sacramento", "Kings"...
    );
    let mut teamIds = HashMap::new();
    for (idx, &team) in teams.iter() {
      teamIds.insert(team, idx);
    }
    let mut franchiseTeams = SmallIntMap::new();
    for (idx, &franchise) in franchises.iter() {
      match franchises.iter().max_by({ |&(idx2, team)|
        if idx == idx2 {
          match teams.find(team) {
            Some(team) => team.year,
            None => i32::MIN
          }
        } else { i32::MIN }
      }) {
        Some((latest_team, _)) => franchiseTeams.insert(franchise, latest_team),
        None => false
      };
    }
    TeamDatabase {
      teams: teams,
      teamIds: teamIds,
      franchises: franchises,
      franchiseTeams: franchiseTeams,
      locations: locations
    }
  }
}

fn main() {
  let db = TeamDatabase::make();
  let oldTeam = Team {name: "Hornets", year: 2010};
  let oldTeamId = db.teamIds.find(&oldTeam);
  let oldTeamLoc = oldTeamId.and_then({ |id| db.locations.find(id)}).map_or("<not found>", { |&loc| loc});
  let franchise = oldTeamId.and_then({ |id| db.franchises.find(id)});
  let newTeamId = franchise.and_then({ |id| db.franchiseTeams.find(id)});
  let newTeam = newTeamId.and_then({ |id| db.teams.find(id) });
  let newTeamLoc = newTeamId.and_then({ |id| db.locations.find(id)}).map_or("<not found>", {|&loc| loc});
  println!(
    "The {} {} of {} became the {} {} in {}.",
    oldTeamLoc, oldTeam.name, oldTeam.year,
    newTeamLoc, newTeam.map_or("<not found>", {|team| team.name}), newTeam.map_or(i32::MIN, {|team| team.year}));
}