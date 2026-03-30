use hdk::prelude::*;
use suggestions_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSuggestionInput {
    pub prompt_hash: ActionHash,
    pub content: String,
    pub ai_model: Option<String>,
}

#[hdk_extern]
pub fn create_suggestion(input: CreateSuggestionInput) -> ExternResult<ActionHash> {
    let suggestion = Suggestion {
        prompt_hash: input.prompt_hash.clone(),
        content: input.content,
        ai_model: input.ai_model,
    };
    let action_hash = create_entry(&EntryTypes::Suggestion(suggestion))?;

    create_link(
        input.prompt_hash,
        action_hash.clone(),
        LinkTypes::PromptToSuggestions,
        (),
    )?;

    let agent = agent_info()?.agent_latest_pubkey;
    create_link(
        agent,
        action_hash.clone(),
        LinkTypes::AgentToSuggestions,
        (),
    )?;

    Ok(action_hash)
}

#[hdk_extern]
pub fn get_suggestions_for_prompt(prompt_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links = get_links(prompt_hash, LinkTypes::PromptToSuggestions, None)?;

    let records: Vec<Record> = links
        .into_iter()
        .filter_map(|link| {
            let hash = ActionHash::try_from(link.target).ok()?;
            get(hash, GetOptions::default()).ok().flatten()
        })
        .collect();

    Ok(records)
}

#[hdk_extern]
pub fn get_agent_suggestions(agent: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(agent, LinkTypes::AgentToSuggestions, None)?;

    let records: Vec<Record> = links
        .into_iter()
        .filter_map(|link| {
            let hash = ActionHash::try_from(link.target).ok()?;
            get(hash, GetOptions::default()).ok().flatten()
        })
        .collect();

    Ok(records)
}
