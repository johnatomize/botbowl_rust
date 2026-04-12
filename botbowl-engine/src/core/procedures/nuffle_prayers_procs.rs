use serde::{Deserialize, Serialize};

use crate::core::{
    dices::{D6, D6Target, D16, RequestedRoll, RollResult, RollTarget},
    gamestate::GameState,
    model::{Action, AvailableActions, BallState, PlayerID, Position, ProcInput, ProcState, Procedure, TeamType},
    procedures::{AnyProc, ball_procs, casualty_procs},
    table::{PosAT, Skill, TemporarySkill},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrayersToNuffle {
    team: TeamType,
}
impl PrayersToNuffle {
    pub fn new(team: TeamType) -> AnyProc {
        AnyProc::PrayersToNuffle(PrayersToNuffle { team })
    }
}
impl Procedure for PrayersToNuffle {
    fn step(&mut self, game_state: &mut GameState, input: ProcInput) -> ProcState {
        let prayers_to_nuffles_roll = match input {
            ProcInput::Nothing => {
                return ProcState::NeedRoll(RequestedRoll::D16);
            }
            ProcInput::Roll(RollResult::D16(prayers_to_nuffle_roll)) => prayers_to_nuffle_roll,
            _ => panic!("Unexpected input {:?}", input),
        };
        let mut procs: Vec<AnyProc> = Vec::new();
        match prayers_to_nuffles_roll {
            D16::One => {
                game_state.info.trapdoors_active = true;
            }
            D16::Two => {
                procs.push(FirendsWithTheRef::new(self.team));
            }
            D16::Three => {
                procs.push(Stiletto::new(self.team));
            }
            D16::Four => {
                procs.push(IronMan::new(self.team));
            }
            D16::Five => {
                procs.push(KnuckleDusters::new(self.team));
            }
            D16::Six => {
                // Todo: implement Bad Habits. The rules for Bad Habits are:
                // Randomly select D3 opposition players that are available to play during this drive and that do not have the Loner (X+) trait. 
                // Until the end of this drive, those players gain the Loner (2+) trait.
            }
            D16::Seven => {
                // Todo: implement Greasy Cleats. The rules for Greasy Cleats are:
                // Randomly select one opposition player that is available to play during this drive. 
                // That player has had their boots tampered with! Until the end of this drive, their MA is reduced by 1.
            }
            D16::Eight => {
                // Todo: implement Blessed Statue of Nuffle. The rules for Blessed Statue of Nuffle are:
                // Choose one player on your team that is available to play during this drive and that does not have the Loner (X+) trait. 
                // Until the end of this game, that player gains the Pro skill.
            }
            D16::Nine => {
                // Todo: implement Moles under the Pitch. The rules for Moles under the Pitch are:
                // Until the end of this half, apply a -1 modifier every time any player attempts to Rush
                // an extra square (-2 should it occur that both coaches have rolled this result).
            }
            D16::Ten => {
                // Todo: implement Perfect Passing. The rules for Perfect Passing are:
                // Until the end of this game, any player on your team that makes a Completion earns 2 SPP, rather than the usual 1 SPP
            }
            D16::Eleven => {
                // Todo: implement Fan Interaction. The rules for Fan Interaction are:
                // Until the end of this drive, if a player on your team causes a Casualty by pushing an opponent into the crowd, 
                // that player will earn 2 SPP exactly as if they had caused a Casualty by performing a Block action.
            }
            D16::Twelve => {
                // Todo: implement Necessary Violence. The rules for Necessary Violence are:
                // Until the end of this drive, any player on your team that causes a Casualty earns 3 SPP, rather than the usual 2 SPP.
            }
            D16::Thirteen => {
                // Todo: implement Fouling Frenzy. The rules for Fouling Frenzy are:
                // Until the end of this drive, any player on your team that causes a Casualty with a Foul action earns 2 SPP,
                // exactly as if they had caused a Casualty by performing a Block action.
            }
            D16::Fourteen => {
                // Todo: implement Throw a Rock. The rules for Throw a Rock are:
                // Until the end of this drive, should an opposition player Stall, at the end of their team turn you may roll a D6. 
                // On a roll of 5+, an angry fan throws a rock at that player. The player is immediately Knocked Down.
            }
            D16::Fifteen => {
                // Todo: implement Under Scrutiny. The rules for Under Scrutiny are:
                // Until the end of this half, any player on the opposing team that commits a Foul action is automatically seen by the referee, 
                // even if a natural double is not rolled.
            }
            D16::Sixteen => {
                // Todo: implement Intensive Training. The rules for Intensive Training are:
                // Randomly select one player on your team that is available to play during this drive and that does not have the Loner (X+) trait. 
                //Until the end of this game, that player gains a single Primary skill of your choice.
            }
        }
        ProcState::from(procs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrapdoorCheck {
    id: PlayerID,
    target: D6Target,
    on_safe_procs: Vec<AnyProc>,
}
impl TrapdoorCheck {
    pub fn new(id: PlayerID, target: D6Target) -> AnyProc {
        Self::new_with_on_safe_procs(id, target, Vec::new())
    }

    pub fn new_with_on_safe_procs(
        id: PlayerID,
        target: D6Target,
        on_safe_procs: Vec<AnyProc>,
    ) -> AnyProc {
        AnyProc::TrapdoorCheck(TrapdoorCheck {
            id,
            target,
            on_safe_procs,
        })
    }
}
impl Procedure for TrapdoorCheck {
    fn step(&mut self, game_state: &mut GameState, input: ProcInput) -> ProcState {
        if !game_state.info.trapdoors_active {
            return ProcState::Done;
        }

        match input {
            ProcInput::Nothing => ProcState::NeedRoll(RequestedRoll::D6),
            ProcInput::Roll(RollResult::D6(roll)) if self.target.is_success(roll) => {
                ProcState::from(std::mem::take(&mut self.on_safe_procs))
            }
            ProcInput::Roll(RollResult::D6(D6::One)) => {
                //FAIL
                let mut procs: Vec<AnyProc> = Vec::new();
                let player_position = match game_state.get_player(self.id) {
                    Ok(player_) => player_.position,
                    Err(_) => panic!("Player with id {:?} not found.", self.id),
                };

                if matches!(game_state.ball, BallState::Carried(carrier_id) if carrier_id == self.id)
                {
                    game_state.ball = BallState::InAir(player_position);
                    procs.push(ball_procs::Bounce::new());
                }
                procs.push(casualty_procs::Injury::new_crowd(self.id));
                ProcState::from(procs)
            }
            _ => panic!("Unexpected input {:?}", input),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirendsWithTheRef {
    team: TeamType,
}
impl FirendsWithTheRef {
    pub fn new(team: TeamType) -> AnyProc {
        AnyProc::FirendsWithTheRef(FirendsWithTheRef { team })
    }
}
impl Procedure for FirendsWithTheRef {
    fn step(&mut self, game_state: &mut GameState, input: ProcInput) -> ProcState {
        match input {
            ProcInput::Nothing => {
                match self.team {
                    TeamType::Home => game_state.home.activate_friends_with_the_ref(),
                    TeamType::Away => game_state.away.activate_friends_with_the_ref(),
                }
                ProcState::Done
            }
            _ => panic!("Unexpected input {:?}", input),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stiletto {
    team: TeamType,
}
impl Stiletto { 
    fn new(team: TeamType) -> AnyProc {
        AnyProc::Stiletto(Stiletto {team})
    }

    fn eligible_players(&self, game_state: &GameState) -> Vec<PlayerID> {
        game_state
            .get_players_on_pitch_in_team(self.team)
            .filter(|player| !player.has_skill(Skill::Loner3) && !player.has_skill(Skill::Loner4))
            .map(|player| player.id)
            .collect()
    }
}
impl Procedure for Stiletto {
    fn step(&mut self, game_state: &mut GameState, input: ProcInput) -> ProcState {
        let eligible_players = self.eligible_players(game_state);

        if eligible_players.is_empty() {
            return ProcState::Done;
        }

        match input {
            ProcInput::Nothing => ProcState::NeedRoll(RequestedRoll::D16),
            ProcInput::Roll(RollResult::D16(roll)) => {
                let index = roll as usize - 1;
                let Some(id) = eligible_players.get(index).copied() else {
                    return ProcState::NeedRoll(RequestedRoll::D16);
                };

                game_state
                    .get_mut_player(id)
                    .expect("eligible player must still be on the pitch")
                    .stats
                    .give_temporary_skill(TemporarySkill::Stab);
                ProcState::Done
            }
            _ => panic!("Unexpected input {:?}", input),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IronMan {
    team: TeamType
}
impl IronMan {
    fn new(team: TeamType) -> AnyProc {
        AnyProc::IronMan(IronMan {team})
    }

    fn eligible_positions(&self, game_state: &GameState) -> Vec<Position> {
        game_state
            .get_players_on_pitch_in_team(self.team)
            .filter(|player| {
                !player.has_skill(Skill::Loner3)
                    && !player.has_skill(Skill::Loner4)
                    && player.stats.av() < 11
            })
            .map(|player| player.position)
            .collect()
    }
}
impl Procedure for IronMan {
    fn step(&mut self, game_state: &mut GameState, input: ProcInput) -> ProcState {
        let eligible_positions = self.eligible_positions(game_state);

        if eligible_positions.is_empty() {
            return ProcState::Done;
        }

        match input {
            ProcInput::Nothing => {
                let mut aa = AvailableActions::new(self.team);
                aa.insert_positional(PosAT::SelectPosition, eligible_positions);
                ProcState::NeedAction(aa)
            }
            ProcInput::Action(Action::Positional(PosAT::SelectPosition, pos)) => {
                if !eligible_positions.contains(&pos) {
                    return ProcState::NeedAction({
                        let mut aa = AvailableActions::new(self.team);
                        aa.insert_positional(PosAT::SelectPosition, eligible_positions);
                        aa
                    });
                }

                let id = game_state
                    .get_player_id_at(pos)
                    .expect("eligible position must contain a player");
                game_state
                    .get_mut_player(id)
                    .expect("eligible player must still be on the pitch")
                    .stats
                    .add_temporary_av(1);
                ProcState::Done
            }
            _ => panic!("Unexpected input {:?}", input),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnuckleDusters {
    team: TeamType
}
impl KnuckleDusters {
    fn new(team: TeamType) -> AnyProc {
        AnyProc::KnuckleDusters(KnuckleDusters {team})
    }

    fn eligible_players(&self, game_state: &GameState) -> Vec<PlayerID> {
        game_state
            .get_players_on_pitch_in_team(self.team)
            .filter(|player| !player.has_skill(Skill::Loner3) && !player.has_skill(Skill::Loner4))
            .map(|player| player.id)
            .collect()
    }
}
impl Procedure for KnuckleDusters {
    fn step(&mut self, game_state: &mut GameState, input: ProcInput) -> ProcState {
        let eligible_players = self.eligible_players(game_state);

        if eligible_players.is_empty() {
            return ProcState::Done;
        }

        match input {
            ProcInput::Nothing => ProcState::NeedRoll(RequestedRoll::D16),
            ProcInput::Roll(RollResult::D16(roll)) => {
                let index = roll as usize - 1;
                let Some(id) = eligible_players.get(index).copied() else {
                    return ProcState::NeedRoll(RequestedRoll::D16);
                };

                game_state
                    .get_mut_player(id)
                    .expect("eligible player must still be on the pitch")
                    .stats
                    .give_temporary_skill(TemporarySkill::MightyBlow1);
                ProcState::Done
            }
            _ => panic!("Unexpected input {:?}", input),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        dices::{RequestedRoll, RollResult, D16, D6},
        gamestate::{GameState, GameStateBuilder},
        model::{Action, PlayerID, PlayerStats, Position, ProcInput, TeamType},
        procedures::Ejection,
        table::SimpleAT,
    };

    use super::*;

    mod friends_with_the_ref {
        use super::*;

        fn activate_friends_with_the_ref(state: &mut GameState, team: TeamType) {
            let mut prayer = match PrayersToNuffle::new(team) {
                AnyProc::PrayersToNuffle(proc) => proc,
                _ => unreachable!(),
            };

            assert!(matches!(
                prayer.step(state, ProcInput::Nothing),
                ProcState::NeedRoll(RequestedRoll::D16)
            ));

            let ProcState::DoneNewProcs(mut procs) =
                prayer.step(state, ProcInput::Roll(RollResult::D16(D16::Two)))
            else {
                panic!("Friends with the Ref should enqueue its activator proc");
            };

            assert_eq!(procs.len(), 1);
            let AnyProc::FirendsWithTheRef(mut effect) = procs.pop().unwrap() else {
                panic!("Expected Friends with the Ref activator proc");
            };

            assert!(matches!(effect.step(state, ProcInput::Nothing), ProcState::Done));
        }

        fn build_single_player_state(team: TeamType, position: Position) -> GameState {
            let mut builder = GameStateBuilder::new();
            match team {
                TeamType::Home => {
                    builder.add_home_player(position);
                }
                TeamType::Away => {
                    builder.add_away_player(position);
                }
            }
            builder.build()
        }

        fn new_foul_ejection(id: PlayerID) -> Ejection {
            match Ejection::new_foul(id) {
                AnyProc::Ejection(proc) => proc,
                _ => unreachable!(),
            }
        }

        fn resolve_foul_argue_the_call(state: &mut GameState, id: PlayerID, roll: D6) {
            let mut ejection = new_foul_ejection(id);

            let proc_state = ejection.step(state, ProcInput::Nothing);
            assert!(matches!(
                proc_state,
                ProcState::NeedAction(aa)
                    if aa.is_legal_action(Action::Simple(SimpleAT::ArgueTheCall))
                        && aa.is_legal_action(Action::Simple(SimpleAT::DontArgueTheCall))
            ));

            assert!(matches!(
                ejection.step(
                    state,
                    ProcInput::Action(Action::Simple(SimpleAT::ArgueTheCall)),
                ),
                ProcState::NeedRoll(RequestedRoll::D6)
            ));

            assert!(matches!(
                ejection.step(state, ProcInput::Roll(RollResult::D6(roll))),
                ProcState::Done
            ));
        }

        #[test]
        fn should_be_removed_at_the_end_of_the_drive() {
            let start_pos = Position::new((2, 5));
            let td_pos = Position::new((1, 5));
            let mut state = GameStateBuilder::new()
                .add_home_player(start_pos)
                .add_ball_pos(start_pos)
                .build();

            activate_friends_with_the_ref(&mut state, TeamType::Home);

            state.step_positional(crate::core::table::PosAT::StartMove, start_pos);
            state.step_positional(crate::core::table::PosAT::Move, td_pos);

            assert_eq!(state.home.score, 1);

            let post_drive_pos = Position::new((5, 5));
            let id = state
                .add_new_player_to_field(PlayerStats::new_lineman(TeamType::Home), post_drive_pos)
                .unwrap();

            resolve_foul_argue_the_call(&mut state, id, D6::Five);

            assert_eq!(state.get_player_id_at(post_drive_pos), None);
            assert!(state.info.turnover);
            assert!(state.home.can_argue_the_call());
            assert!(state.get_dugout().any(|player| {
                player.place == crate::core::model::DugoutPlace::Ejected
                    && player.stats.team == TeamType::Home
            }));
        }

        #[test]
        fn argue_the_call_should_be_correctly_modified_on_roll_of_five_or_six() {
            let start_pos = Position::new((5, 5));

            for roll in [D6::Five, D6::Six] {
                let mut state = build_single_player_state(TeamType::Home, start_pos);
                activate_friends_with_the_ref(&mut state, TeamType::Home);

                let id = state.get_player_id_at(start_pos).unwrap();
                resolve_foul_argue_the_call(&mut state, id, roll);

                assert_eq!(state.get_player_id_at(start_pos), Some(id));
                assert!(state.info.turnover);
                assert!(state.home.can_argue_the_call());
                assert!(state.get_dugout().next().is_none());
            }

            let mut control_state = build_single_player_state(TeamType::Away, start_pos);
            activate_friends_with_the_ref(&mut control_state, TeamType::Home);

            let away_id = control_state.get_player_id_at(start_pos).unwrap();
            resolve_foul_argue_the_call(&mut control_state, away_id, D6::Five);

            assert_eq!(control_state.get_player_id_at(start_pos), None);
            assert!(control_state.info.turnover);
            assert!(control_state.away.can_argue_the_call());
            assert!(control_state.get_dugout().any(|player| {
                player.place == crate::core::model::DugoutPlace::Ejected
                    && player.stats.team == TeamType::Away
            }));
        }

        #[test]
        fn argue_the_call_should_be_correctly_modified_on_roll_of_two_to_four() {
            let start_pos = Position::new((5, 5));

            for roll in [D6::Two, D6::Three, D6::Four] {
                let mut state = build_single_player_state(TeamType::Home, start_pos);
                activate_friends_with_the_ref(&mut state, TeamType::Home);

                let id = state.get_player_id_at(start_pos).unwrap();
                resolve_foul_argue_the_call(&mut state, id, roll);

                assert_eq!(state.get_player_id_at(start_pos), None);
                assert!(state.info.turnover);
                assert!(state.home.can_argue_the_call());
                assert!(state.get_dugout().any(|player| {
                    player.place == crate::core::model::DugoutPlace::Ejected
                        && player.stats.team == TeamType::Home
                }));
            }
        }
    }

    mod stiletto {
        use crate::core::{model::DugoutPlace, table::{Skill, TemporarySkill}};

        use super::*;

        fn activate_stiletto(state: &mut GameState, team: TeamType, selection_roll: D16) {
            let mut prayer = match PrayersToNuffle::new(team) {
                AnyProc::PrayersToNuffle(proc) => proc,
                _ => unreachable!(),
            };

            assert!(matches!(
                prayer.step(state, ProcInput::Nothing),
                ProcState::NeedRoll(RequestedRoll::D16)
            ));

            let ProcState::DoneNewProcs(mut procs) =
                prayer.step(state, ProcInput::Roll(RollResult::D16(D16::Three)))
            else {
                panic!("Stiletto should enqueue its activator proc");
            };

            assert_eq!(procs.len(), 1);
            let AnyProc::Stiletto(mut effect) = procs.pop().unwrap() else {
                panic!("Expected Stiletto activator proc");
            };

            assert!(matches!(
                effect.step(state, ProcInput::Nothing),
                ProcState::NeedRoll(RequestedRoll::D16)
            ));
            assert!(matches!(
                effect.step(state, ProcInput::Roll(RollResult::D16(selection_roll))),
                ProcState::Done
            ));
        }

        #[test]
        fn only_players_on_the_pitch_available_for_selection() {
            let start_pos = Position::new((5, 5));
            let mut state = GameStateBuilder::empty_state();

            let on_pitch_id = state
                .add_new_player_to_field(PlayerStats::new_lineman(TeamType::Home), start_pos)
                .unwrap();
            state.dugout_add_new_player(PlayerStats::new_lineman(TeamType::Home), DugoutPlace::Reserves);

            activate_stiletto(&mut state, TeamType::Home, D16::One);

            assert!(state.get_player(on_pitch_id).unwrap().has_temporary_skill(TemporarySkill::Stab));
            assert!(state
                .get_dugout()
                .filter(|player| player.stats.team == TeamType::Home)
                .all(|player| !player.stats.has_temporary_skill(TemporarySkill::Stab)));
        }

        #[test]
        fn players_with_loner_skill_not_selectable() {
            let loner_pos = Position::new((5, 5));
            let normal_pos = Position::new((6, 5));
            let mut state = GameStateBuilder::empty_state();

            let mut loner_stats = PlayerStats::new_lineman(TeamType::Home);
            loner_stats.give_skill(Skill::Loner3);
            let loner_id = state.add_new_player_to_field(loner_stats, loner_pos).unwrap();
            let normal_id = state
                .add_new_player_to_field(PlayerStats::new_lineman(TeamType::Home), normal_pos)
                .unwrap();

            activate_stiletto(&mut state, TeamType::Home, D16::One);

            assert!(!state.get_player(loner_id).unwrap().has_temporary_skill(TemporarySkill::Stab));
            assert!(state.get_player(normal_id).unwrap().has_temporary_skill(TemporarySkill::Stab));
        }

        #[test]
        fn skill_is_lost_at_end_of_drive() {
            let start_pos = Position::new((2, 5));
            let td_pos = Position::new((1, 5));
            let mut state = GameStateBuilder::new()
                .add_home_player(start_pos)
                .add_ball_pos(start_pos)
                .build();

            let id = state.get_player_id_at(start_pos).unwrap();
            activate_stiletto(&mut state, TeamType::Home, D16::One);
            assert!(state.get_player(id).unwrap().has_temporary_skill(TemporarySkill::Stab));

            state.step_positional(crate::core::table::PosAT::StartMove, start_pos);
            state.step_positional(crate::core::table::PosAT::Move, td_pos);

            assert_eq!(state.home.score, 1);
            assert!(state
                .get_dugout()
                .filter(|player| player.stats.team == TeamType::Home)
                .all(|player| !player.stats.has_temporary_skill(TemporarySkill::Stab)));
        }
    }

    mod iron_man {
        use crate::core::{model::DugoutPlace, table::{PosAT, Skill}};

        use super::*;

        fn activate_iron_man(state: &mut GameState, team: TeamType, position: Position) {
            let mut prayer = match PrayersToNuffle::new(team) {
                AnyProc::PrayersToNuffle(proc) => proc,
                _ => unreachable!(),
            };

            assert!(matches!(
                prayer.step(state, ProcInput::Nothing),
                ProcState::NeedRoll(RequestedRoll::D16)
            ));

            let ProcState::DoneNewProcs(mut procs) =
                prayer.step(state, ProcInput::Roll(RollResult::D16(D16::Four)))
            else {
                panic!("Iron Man should enqueue its activator proc");
            };

            assert_eq!(procs.len(), 1);
            let AnyProc::IronMan(mut effect) = procs.pop().unwrap() else {
                panic!("Expected Iron Man activator proc");
            };

            let ProcState::NeedAction(aa) = effect.step(state, ProcInput::Nothing) else {
                panic!("Iron Man should require selecting an eligible player");
            };
            assert!(aa.is_legal_action(Action::Positional(PosAT::SelectPosition, position)));

            assert!(matches!(
                effect.step(
                    state,
                    ProcInput::Action(Action::Positional(PosAT::SelectPosition, position)),
                ),
                ProcState::Done
            ));
        }

        #[test]
        fn only_players_on_the_pitch_available_for_selection() {
            let start_pos = Position::new((5, 5));
            let mut state = GameStateBuilder::empty_state();

            let on_pitch_id = state
                .add_new_player_to_field(PlayerStats::new_lineman(TeamType::Home), start_pos)
                .unwrap();
            state.dugout_add_new_player(PlayerStats::new_lineman(TeamType::Home), DugoutPlace::Reserves);

            activate_iron_man(&mut state, TeamType::Home, start_pos);

            assert_eq!(state.get_player(on_pitch_id).unwrap().stats.av(), 9);
            assert!(state
                .get_dugout()
                .filter(|player| player.stats.team == TeamType::Home)
                .all(|player| player.stats.av() == player.stats.av));
        }

        #[test]
        fn players_with_loner_skill_not_selectable() {
            let loner_pos = Position::new((5, 5));
            let normal_pos = Position::new((6, 5));
            let mut state = GameStateBuilder::empty_state();

            let mut loner_stats = PlayerStats::new_lineman(TeamType::Home);
            loner_stats.give_skill(Skill::Loner3);
            let loner_id = state.add_new_player_to_field(loner_stats, loner_pos).unwrap();
            let normal_id = state
                .add_new_player_to_field(PlayerStats::new_lineman(TeamType::Home), normal_pos)
                .unwrap();

            let mut prayer = match PrayersToNuffle::new(TeamType::Home) {
                AnyProc::PrayersToNuffle(proc) => proc,
                _ => unreachable!(),
            };

            assert!(matches!(
                prayer.step(&mut state, ProcInput::Nothing),
                ProcState::NeedRoll(RequestedRoll::D16)
            ));

            let ProcState::DoneNewProcs(mut procs) =
                prayer.step(&mut state, ProcInput::Roll(RollResult::D16(D16::Four)))
            else {
                panic!("Iron Man should enqueue its activator proc");
            };

            let AnyProc::IronMan(mut effect) = procs.pop().unwrap() else {
                panic!("Expected Iron Man activator proc");
            };

            let ProcState::NeedAction(aa) = effect.step(&mut state, ProcInput::Nothing) else {
                panic!("Iron Man should require selecting an eligible player");
            };
            assert!(!aa.is_legal_action(Action::Positional(PosAT::SelectPosition, loner_pos)));
            assert!(aa.is_legal_action(Action::Positional(PosAT::SelectPosition, normal_pos)));

            assert!(matches!(
                effect.step(
                    &mut state,
                    ProcInput::Action(Action::Positional(PosAT::SelectPosition, normal_pos)),
                ),
                ProcState::Done
            ));

            assert_eq!(state.get_player(loner_id).unwrap().stats.av(), 8);
            assert_eq!(state.get_player(normal_id).unwrap().stats.av(), 9);
        }

        #[test]
        fn armor_value_is_lost_at_end_of_drive() {
            let start_pos = Position::new((2, 5));
            let td_pos = Position::new((1, 5));
            let mut state = GameStateBuilder::new()
                .add_home_player(start_pos)
                .add_ball_pos(start_pos)
                .build();

            let id = state.get_player_id_at(start_pos).unwrap();
            activate_iron_man(&mut state, TeamType::Home, start_pos);
            assert_eq!(state.get_player(id).unwrap().stats.av(), 9);

            state.step_positional(crate::core::table::PosAT::StartMove, start_pos);
            state.step_positional(crate::core::table::PosAT::Move, td_pos);

            assert_eq!(state.home.score, 1);
            assert!(state
                .get_dugout()
                .filter(|player| player.stats.team == TeamType::Home)
                .all(|player| player.stats.av() == player.stats.av));
        }

        #[test]
        fn player_with_armor_value_11_is_not_elegible() {
            let maxed_pos = Position::new((5, 5));
            let normal_pos = Position::new((6, 5));
            let mut state = GameStateBuilder::empty_state();

            let mut maxed_stats = PlayerStats::new_lineman(TeamType::Home);
            maxed_stats.av = 11;
            let maxed_id = state.add_new_player_to_field(maxed_stats, maxed_pos).unwrap();
            let normal_id = state
                .add_new_player_to_field(PlayerStats::new_lineman(TeamType::Home), normal_pos)
                .unwrap();

            let mut prayer = match PrayersToNuffle::new(TeamType::Home) {
                AnyProc::PrayersToNuffle(proc) => proc,
                _ => unreachable!(),
            };

            assert!(matches!(
                prayer.step(&mut state, ProcInput::Nothing),
                ProcState::NeedRoll(RequestedRoll::D16)
            ));

            let ProcState::DoneNewProcs(mut procs) =
                prayer.step(&mut state, ProcInput::Roll(RollResult::D16(D16::Four)))
            else {
                panic!("Iron Man should enqueue its activator proc");
            };

            let AnyProc::IronMan(mut effect) = procs.pop().unwrap() else {
                panic!("Expected Iron Man activator proc");
            };

            let ProcState::NeedAction(aa) = effect.step(&mut state, ProcInput::Nothing) else {
                panic!("Iron Man should require selecting an eligible player");
            };
            assert!(!aa.is_legal_action(Action::Positional(PosAT::SelectPosition, maxed_pos)));
            assert!(aa.is_legal_action(Action::Positional(PosAT::SelectPosition, normal_pos)));

            assert!(matches!(
                effect.step(
                    &mut state,
                    ProcInput::Action(Action::Positional(PosAT::SelectPosition, normal_pos)),
                ),
                ProcState::Done
            ));

            assert_eq!(state.get_player(maxed_id).unwrap().stats.av(), 11);
            assert_eq!(state.get_player(normal_id).unwrap().stats.av(), 9);

            let mut all_maxed_state = GameStateBuilder::empty_state();
            let mut all_maxed_stats = PlayerStats::new_lineman(TeamType::Home);
            all_maxed_stats.av = 11;
            all_maxed_state
                .add_new_player_to_field(all_maxed_stats, maxed_pos)
                .unwrap();

            let mut effect = match IronMan::new(TeamType::Home) {
                AnyProc::IronMan(proc) => proc,
                _ => unreachable!(),
            };

            assert!(matches!(
                effect.step(&mut all_maxed_state, ProcInput::Nothing),
                ProcState::Done
            ));
        }
    }

    mod knuckle_dusters {
        use crate::core::{model::DugoutPlace, table::{Skill, TemporarySkill}};

        use super::*;

        fn activate_knuckle_dusters(state: &mut GameState, team: TeamType, selection_roll: D16) {
            let mut prayer = match PrayersToNuffle::new(team) {
                AnyProc::PrayersToNuffle(proc) => proc,
                _ => unreachable!(),
            };

            assert!(matches!(
                prayer.step(state, ProcInput::Nothing),
                ProcState::NeedRoll(RequestedRoll::D16)
            ));

            let ProcState::DoneNewProcs(mut procs) =
                prayer.step(state, ProcInput::Roll(RollResult::D16(D16::Five)))
            else {
                panic!("Knuckle Dusters should enqueue its activator proc");
            };

            assert_eq!(procs.len(), 1);
            let AnyProc::KnuckleDusters(mut effect) = procs.pop().unwrap() else {
                panic!("Expected Knuckle Dusters activator proc");
            };

            assert!(matches!(
                effect.step(state, ProcInput::Nothing),
                ProcState::NeedRoll(RequestedRoll::D16)
            ));
            assert!(matches!(
                effect.step(state, ProcInput::Roll(RollResult::D16(selection_roll))),
                ProcState::Done
            ));
        }

        #[test]
        fn only_players_on_the_pitch_available_for_selection() {
            let start_pos = Position::new((5, 5));
            let mut state = GameStateBuilder::empty_state();

            let on_pitch_id = state
                .add_new_player_to_field(PlayerStats::new_lineman(TeamType::Home), start_pos)
                .unwrap();
            state.dugout_add_new_player(
                PlayerStats::new_lineman(TeamType::Home),
                DugoutPlace::Reserves,
            );

            activate_knuckle_dusters(&mut state, TeamType::Home, D16::One);

            assert!(state
                .get_player(on_pitch_id)
                .unwrap()
                .has_temporary_skill(TemporarySkill::MightyBlow1));
            assert!(state
                .get_dugout()
                .filter(|player| player.stats.team == TeamType::Home)
                .all(|player| !player.stats.has_temporary_skill(TemporarySkill::MightyBlow1)));
        }

        #[test]
        fn players_with_loner_skill_not_selectable() {
            let loner_pos = Position::new((5, 5));
            let normal_pos = Position::new((6, 5));
            let mut state = GameStateBuilder::empty_state();

            let mut loner_stats = PlayerStats::new_lineman(TeamType::Home);
            loner_stats.give_skill(Skill::Loner3);
            let loner_id = state.add_new_player_to_field(loner_stats, loner_pos).unwrap();
            let normal_id = state
                .add_new_player_to_field(PlayerStats::new_lineman(TeamType::Home), normal_pos)
                .unwrap();

            activate_knuckle_dusters(&mut state, TeamType::Home, D16::One);

            assert!(!state
                .get_player(loner_id)
                .unwrap()
                .has_temporary_skill(TemporarySkill::MightyBlow1));
            assert!(state
                .get_player(normal_id)
                .unwrap()
                .has_temporary_skill(TemporarySkill::MightyBlow1));
        }

        #[test]
        fn skill_is_lost_at_end_of_drive() {
            let start_pos = Position::new((2, 5));
            let td_pos = Position::new((1, 5));
            let mut state = GameStateBuilder::new()
                .add_home_player(start_pos)
                .add_ball_pos(start_pos)
                .build();

            let id = state.get_player_id_at(start_pos).unwrap();
            activate_knuckle_dusters(&mut state, TeamType::Home, D16::One);
            assert!(state
                .get_player(id)
                .unwrap()
                .has_temporary_skill(TemporarySkill::MightyBlow1));

            state.step_positional(crate::core::table::PosAT::StartMove, start_pos);
            state.step_positional(crate::core::table::PosAT::Move, td_pos);

            assert_eq!(state.home.score, 1);
            assert!(state
                .get_dugout()
                .filter(|player| player.stats.team == TeamType::Home)
                .all(|player| !player.stats.has_temporary_skill(TemporarySkill::MightyBlow1)));
        }
    }
}
