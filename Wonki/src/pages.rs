use serde_derive::Deserialize;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use warp::Filter;

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub id: Option<i32>,
    pub deck: Option<String>,
    //pub content: Option<str>,
}

pub fn options() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    homepage_no_auth_filter()
        .or(register_filter())
        .or(login_filter())
        .or(home_filter())
        .or(deck_filter())
        .or(create_filter())
        .or(add_filter())
        .or(viewer_filter())
        .or(latest_filter())
        .or(share_filter())
        .or(addfriend_filter())
        .or(friendpage_filter())
        .or(friendcode_filter())
}

pub fn homepage_no_auth_filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(homepage_no_auth)
}

async fn homepage_no_auth() -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let mut file = match File::open("pages/home.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    Ok(warp::reply::html(content))
}

pub fn register_filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("register").and(warp::get()).and_then(register)
}

async fn register() -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let mut file = match File::open("pages/register.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    Ok(warp::reply::html(content))
}

pub fn login_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("login").and(warp::get()).and_then(login)
}

async fn login() -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let mut file = match File::open("pages/login.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    Ok(warp::reply::html(content))
}

pub fn home_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("home").and(warp::get()).and_then(home)
}

async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let mut file = match File::open("pages/auth_home.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    Ok(warp::reply::html(content))
}

pub fn deck_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("deck")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(deck)
}

async fn deck(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let deckname: &str = match params.get("deck") {
        Some(deck) => &deck[..],
        None => return Err(warp::reject::not_found()),
    };
    let mut file = match File::open("pages/deck.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    let content = content.replace("{{.deckname}}", &deckname);
    Ok(warp::reply::html(content))
}

pub fn create_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("create").and(warp::get()).and_then(create)
}

async fn create() -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let mut file = match File::open("pages/new.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    Ok(warp::reply::html(content))
}

pub fn add_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("add")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(add)
}

async fn add(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let deckname: &str = match params.get("deck") {
        Some(deckb) => &deckb[..],
        None => return Err(warp::reject::not_found()),
    };
    let mut file = match File::open("pages/add.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    let content = content.replace("{{.deckname}}", &deckname);
    Ok(warp::reply::html(content))
}

pub fn viewer_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("view")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and_then(viewer)
}

async fn viewer(opts: ListOptions) -> Result<impl warp::Reply, warp::Rejection> {
    let id = match opts.id {
        Some(numeral) => numeral,
        None => return Err(warp::reject::not_found()),
    };

    let deckname: String = match opts.deck {
        Some(decka) => decka,
        None => return Err(warp::reject::not_found()),
    };
    let deckname = &deckname[..];
    let mut file = match File::open("pages/viewer.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    let cont_source = &format!("cards?id={id}&deck={deckname}")[..];
    let idstring: &str = &id.to_string()[..];
    let content = content.replace("{{.retrieval}}", &cont_source);
    let content = content.replace("{{.id}}", &idstring);
    Ok(warp::reply::html(content))
}

pub fn latest_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("latest").and(warp::get()).and_then(latest)
}

async fn latest() -> Result<impl warp::Reply, warp::Rejection> {
    let mut file = match File::open("pages/viewer.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    let cont_source = "last";
    let idstring = "latest viewed card";
    let content = content.replace("{{.retrieval}}", &cont_source);
    let content = content.replace("Card {{.id}}", &idstring); //this is kind of ugly, may change it later
    Ok(warp::reply::html(content))
}

pub fn share_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("share")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(share)
}

async fn share(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let deckname: &str = match params.get("deck") {
        Some(deckb) => &deckb[..],
        None => return Err(warp::reject::not_found()),
    };
    let mut file = match File::open("pages/add.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    let header = format!("Share {} with:", deckname);
    let header = &header[..];
    let content = content.replace("{{.deckname}}", &deckname);
    let content = content.replace("Add card:", &header);
    let content = content.replace("/add", "/share");
    let content = content.replace(">content:<", ">username:<");
    Ok(warp::reply::html(content))
}

pub fn addfriend_filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("addfriend")
        .and(warp::get())
        .and_then(addfriend)
}

async fn addfriend() -> Result<impl warp::Reply, warp::Rejection> {
    let mut file = match File::open("pages/friendadder.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    Ok(warp::reply::html(content))
}

pub fn friendpage_filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("friendpage")
        .and(warp::get())
        .and_then(friendpage)
}

async fn friendpage() -> Result<impl warp::Reply, warp::Rejection> {
    // Load the HTML content from the file
    let mut file = match File::open("pages/friendpage.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    Ok(warp::reply::html(content))
}

pub fn friendcode_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("gettoken").and(warp::get()).and_then(friendcode)
}

async fn friendcode() -> Result<impl warp::Reply, warp::Rejection> {
    let mut file = match File::open("pages/viewer.html").await {
        Ok(file) => file,
        Err(_) => return Err(warp::reject::not_found()),
    };
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content).await {
        return Err(warp::reject::not_found());
    }
    let cont_source = "friendcode";
    let idstring = "Your Friendcode";
    let content = content.replace("{{.retrieval}}", &cont_source);
    let content = content.replace("Card {{.id}}", &idstring); //this is kind of ugly, may change it later
    Ok(warp::reply::html(content))
}
