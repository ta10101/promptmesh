use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Prompt {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Prompt(Prompt),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    AllPrompts,
    AgentToPrompts,
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry { app_entry, .. } => match app_entry {
                EntryTypes::Prompt(prompt) => {
                    if prompt.title.is_empty() {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Prompt title cannot be empty".into(),
                        ));
                    }
                    if prompt.content.is_empty() {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Prompt content cannot be empty".into(),
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
