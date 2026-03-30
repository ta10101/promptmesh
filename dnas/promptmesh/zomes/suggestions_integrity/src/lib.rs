use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Suggestion {
    pub prompt_hash: ActionHash,
    pub content: String,
    pub ai_model: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Suggestion(Suggestion),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    PromptToSuggestions,
    AgentToSuggestions,
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry { app_entry, .. } => match app_entry {
                EntryTypes::Suggestion(suggestion) => {
                    if suggestion.content.is_empty() {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Suggestion content cannot be empty".into(),
                        ));
                    }
                    Ok(ValidateCallbackResult::Valid)
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        _ => Ok(ValidateCallbackResult::Valid),
    }
}
