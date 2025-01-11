use leptos::*;
use reqwest::Client;
use web_sys::console;

use crate::models::model_persons::{MyPerson, PersonId, NewPerson};

const BASE_URL_BOOKS: &str = "http://localhost:8000/persons";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ QUERIES ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


// * 💁 From the client 💡
pub async fn get_all_persons() -> Result<Vec<MyPerson>, reqwest::Error> {
    let response = reqwest::get(BASE_URL_BOOKS).await?;
    let persons: Vec<MyPerson> = response.json().await?;
    
    Ok(persons)
}

pub async fn get_person_by_id(id: PersonId) -> Result<Option<MyPerson>, ServerFnError> {
    let url = format!("{}/{}", BASE_URL_BOOKS, id);
    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        let person: MyPerson = response.json().await?;
        Ok(Some(person))
    } else {
        Err(ServerFnError::ServerError(format!("Failed to retrieve person with ID: {}", id)))
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn delete_person(id: PersonId) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let url = format!("{}/{}", BASE_URL_BOOKS, id); 
    // * 💁 I needed to implement fmt::Display for ServerPersonId to use it directly 
    // *    and not have id.0 since it's a tuple

    let response = client.delete(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to delete person"))
    }
}


pub async fn add_person(new_person: NewPerson) -> Result<MyPerson, anyhow::Error> {
    let client = Client::new();
    let url = BASE_URL_BOOKS;

    // Use web_sys to log to the browser console
    console::log_1(&format!("new_person: {:?}", new_person).into());

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .json(&new_person)
        .send().await?;

    if response.status().is_success() {
        let person: MyPerson = response.json().await?;
        Ok(person)
    } else {
        Err(anyhow::anyhow!("Failed to add person"))
    }
}
