pub mod bidder;
pub mod calc;
pub mod conditions;
pub mod config;
pub mod llm;
pub mod llm_bidder;
pub mod pilot;
pub mod prompt;
pub mod protocol;
pub mod qre;
pub mod qre_fit;
pub mod quiz;
pub mod record;
pub mod rules;
pub mod seeds;
pub mod simulation;
pub mod solver;
pub mod usage;
pub mod world;

pub const EVENT_NAMESPACE: &str = "exp-dollar-auction-escalation";
pub const LEGACY_EVENT_NAMESPACE: &str = "dollar-auction-escalation";

pub(crate) fn is_experiment_event(schema: &str, kind: &str) -> bool {
    let Some(schema) = schema.strip_prefix("x.") else {
        return false;
    };
    [EVENT_NAMESPACE, LEGACY_EVENT_NAMESPACE]
        .iter()
        .any(|namespace| {
            schema
                .strip_prefix(namespace)
                .and_then(|suffix| suffix.strip_prefix('.'))
                == Some(kind)
        })
}
