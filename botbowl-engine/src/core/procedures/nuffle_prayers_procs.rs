use serde::{Deserialize, Serialize};

use crate::core::{
    dices::{D6Target, RequestedRoll, RollResult, RollTarget, D16, D6},
    gamestate::GameState,
    model::{BallState, PlayerID, ProcInput, ProcState, Procedure},
    procedures::{ball_procs, casualty_procs, AnyProc},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrayersToNuffle {}
impl PrayersToNuffle {
    pub fn new() -> AnyProc {
        AnyProc::PrayersToNuffle(PrayersToNuffle {})
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
        let procs: Vec<AnyProc> = Vec::new();
        match prayers_to_nuffles_roll {
            D16::One => {
                game_state.info.trapdoors_active = true;
            }
            D16::Two => {
                // Todo: implement Friends with the ref. The rules for Friends with the Ref are:
                // Until the end of this drive, you may treat a roll of 5 or 6 on the Argue the Call table as a
                // WellWhenYouPutItLikeThat result and a roll of 2-4 as an “I Don’t Care!” result
            }
            D16::Three => {
                //Todo: implement Stiletto. The rules for Stiletto are: 
                // Randomly select one player on your team that is available to play during this drive and that does not have the Loner (X+) trait. 
                // Until the end of this drive, that player gains the Stab trait.
            }
            D16::Four => {
                // Todo: implement Iron Man. The rules for Iron Man are:
                // Choose one player on your team that is available to play during this drive and that does not have the Loner (X+) trait. 
                // Until the end of this game, that player improves their AV by 1, to a maximum of 11+.
            }
            D16::Five => {
                // Todo: implement Knuckle Dusters. The rules for Knuckle Dusters are:
                // Choose one player on your team that is available to play during this drive and that does not have the Loner (X+) trait. 
                // Until the end of this drive, that player gains the Mighty Blow (+1) skill. 
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
