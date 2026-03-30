use hdk::prelude::*;
use prompts_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePromptInput {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[hdk_extern]
pub fn create_prompt(input: CreatePromptInput) -> ExternResult<ActionHash> {
    let prompt = Prompt {
        title: input.title,
        content: input.content,
        tags: input.tags,
    };
    let action_hash = create_entry(&EntryTypes::Prompt(prompt))?;

    let path = Path::from("all_prompts");
    let path_hash = path.path_entry_hash()?;
    create_link(
        path_hash,
        action_hash.clone(),
        LinkTypes::AllPrompts,
        (),
    )?;

    let agent = agent_info()?.agent_latest_pubkey;
    create_link(
        agent,
        action_hash.clone(),
        LinkTypes::AgentToPrompts,
        (),
    )?;

    Ok(action_hash)
}

#[hdk_extern]
pub fn get_prompt(action_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(action_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_all_prompts(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_prompts");
    let path_hash = path.path_entry_hash()?;
    let links = get_links(path_hash, LinkTypes::AllPrompts, None)?;

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
pub fn get_agent_prompts(agent: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(agent, LinkTypes::AgentToPrompts, None)?;

    let records: Vec<Record> = links
        .into_iter()
        .filter_map(|link| {
            let hash = ActionHash::try_from(link.target).ok()?;
            get(hash, GetOptions::default()).ok().flatten()
        })
        .collect();

    Ok(records)
}
