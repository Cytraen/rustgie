use serde::{Deserialize, Serialize};

/// Represents the status and other related information for a challenge that is - or was - available to a player.
/// A challenge is a bonus objective, generally tacked onto Quests or Activities, that provide additional variations on play.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyChallengeStatus {
    /// The progress - including completion status - of the active challenge.
    #[serde(rename = "objective")]
    pub objective: Option<crate::destiny::quests::DestinyObjectiveProgress>,
}
