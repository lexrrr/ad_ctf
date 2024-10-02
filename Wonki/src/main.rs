use inline_python::{python, Context};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
//use std::convert::Infallible;
//use std::env;
use std::sync::{Arc, RwLock};
//use uuid::Uuid;
//use warp::http::StatusCode;
use hex::encode;
use sha2::{Digest, Sha256};
use warp::Filter;
mod pages;
/*
pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}
*/

pub fn hash(password: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    encode(result)
}

pub fn verify(stored_hash: &String, password: &String) -> bool {
    let hashed_password = hash(password);
    stored_hash == &hashed_password
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

mod cards {
    use crate::models::Cardlist;

    use super::models::{Card, Decklist, ListOptions};
    use super::users::{clean, with_db};
    use inline_python::{python, Context};
    use std::collections::HashMap;
    use std::convert::Infallible;
    use std::sync::{Arc, RwLock};
    use warp::http::StatusCode;
    use warp::reply::Reply;
    use warp::Filter;

    pub fn options(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        new_card(sessions_db.clone())
            .or(get_card_content(sessions_db.clone()))
            .or(new_deck(sessions_db.clone()))
            .or(get_last_seen(sessions_db.clone()))
            .or(decks_filter(sessions_db.clone()))
            .or(cards_filter(sessions_db.clone()))
            .or(share_filter(sessions_db.clone()))
            .or(pub_filter())
            .or(token_filter(sessions_db.clone()))
            .or(friend_filter(sessions_db.clone()))
            .or(friendslist_filter(sessions_db.clone()))
    }

    pub fn get_card_content(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("cards")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(return_content)
    }

    pub fn new_deck(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("create")
            .and(warp::post())
            .and(json_body())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(create_deck)
    }

    pub fn new_card(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("add")
            .and(warp::post())
            .and(json_body())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(create_card)
    }

    pub fn get_last_seen(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("last")
            .and(warp::get())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(fetch_latest)
    }

    pub async fn return_content(
        opts: ListOptions,
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        //let opts = opts.unwrap();
        //println!("{:#?}\n\n{:#?}", opts.id, opts.deck);
        let id = opts.id.unwrap_or(1);
        let deck = opts.deck.unwrap();
        let db = sessions_db.read().unwrap();
        match db.get(&session_id) {
            Some(user) => {
                let c: Context = python! {
                    import psycopg2 as sq

                    con = sq.connect(
                        host="postgres",
                        database="wonki-db",
                        user="superuser",
                        password="verysecurepw"
                    )
                    cur = con.cursor()
                    cur.execute("SELECT content FROM cards WHERE id = %s AND deckname = %s", ('id, 'deck))
                    res = cur.fetchone()
                    if res is None:
                        content = "Unable to find"
                    else:
                        content = res[0]
                    cur.execute("UPDATE latest SET content = %s WHERE username = %s", (content,'user))
                    con.commit()
                    cur.execute("SELECT views FROM decks WHERE deckname = %s AND owner = %s", ('deck, 'user))
                    res = cur.fetchone()
                    if res is None:
                        owner = False
                    else:
                        cur.execute("UPDATE decks SET views = %s WHERE deckname = %s and owner = %s", (res[0]+1,'deck, 'user))
                        con.commit()
                        owner = True
                    cur.close()
                    con.close()
                };
                let owner = c.get::<bool>("owner");
                if owner {
                    let result = c.get::<String>("content");
                    //Ok(warp::reply::html(result))
                    let res = warp::reply::html(result);
                    Ok(res.into_response())
                } else {
                    //let result = "Unauthorized!".to_string();
                    //Ok(warp::reply::html(result))
                    //Err(warp::reject::not_found())
                    Ok(
                        warp::redirect::found(warp::http::Uri::from_static("/logout"))
                            .into_response(),
                    )
                }
            }
            None => {
                let result = "No Matches!\n".to_string();
                Ok(warp::reply::html(result).into_response())
                //Err(warp::reject::not_found())
            }
        }
    }

    pub async fn create_card(
        body: Card,
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let content = body.content.unwrap();
        let deck = body.deck.unwrap();
        let db = sessions_db.read().unwrap();
        match db.get(&session_id) {
            Some(user) => {
                let c: Context = python! {
                    import psycopg2 as sq
                    import time

                    con = sq.connect(
                        host="postgres",
                        database="wonki-db",
                        user="superuser",
                        password="verysecurepw"
                    )
                    cur = con.cursor()
                    cur.execute("SELECT cards FROM decks WHERE deckname = %s AND owner = %s", ('deck, 'user))
                    id = cur.fetchone()[0]
                    if id is not None:
                        fail = False
                        thistime = time.time()
                        cur.execute("INSERT INTO cards (id, deckname, content, time) VALUES (%s, %s, %s, %s)", ((id+1), 'deck, 'content, thistime))
                        cur.execute("UPDATE decks SET cards = %s WHERE deckname = %s", ((id+1), 'deck))
                        con.commit()
                    else:
                        fail = True
                    cur.close()
                    con.close()
                };
                let fail = c.get::<bool>("fail");
                if fail{
                    Ok(StatusCode::UNAUTHORIZED)
                }else{
                    Ok(StatusCode::CREATED)
                }
            }
            None => Ok(StatusCode::UNAUTHORIZED),
        }
    }

    pub async fn create_deck(
        body: Card,
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let name = body.deck.unwrap();
            let db = sessions_db.read().unwrap();
            match db.get(&session_id) {
                Some(user) => {
                    let _c: Context = python! {
                        import psycopg2 as sq
                        import time

                        con = sq.connect(
                            host="postgres",
                            database="wonki-db",
                            user="superuser",
                            password="verysecurepw"
                        )
                        cur = con.cursor()
                        thistime = time.time()
                        cur.execute("INSERT INTO decks (deckname, owner, cards, views, time) VALUES (%s, %s, 0, 0, %s)", ('name, 'user, thistime))
                        con.commit()
                        cur.close()
                        con.close()
                    };
                    Ok(StatusCode::CREATED)
                }
                None => Ok(StatusCode::UNAUTHORIZED),
            }
    }

    pub async fn fetch_latest(
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let db = sessions_db.read().unwrap();
        match db.get(&session_id) {
            Some(user) => {
                //let clean_uname = clean(&user);
                let c: Context = python! {
                    import psycopg2 as sq

                    con = sq.connect(
                        host="postgres",
                        database="wonki-db",
                        user="superuser",
                        password="verysecurepw"
                    )
                    cur = con.cursor()
                    cur.execute("SELECT content FROM latest WHERE username = %s", ('user,))
                    res = cur.fetchone()
                    if res is None:
                        content = "Unable to find"
                    else:
                        content = res[0]
                    cur.close()
                    con.close()
                };
                let result = c.get::<String>("content");
                Ok(warp::reply::html(result))
            }
            None => {
                let result = "Nothing found!\n".to_string();
                Ok(warp::reply::html(result))
            }
        }
    }

    pub fn decks_filter(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("decks")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(list_decks)
    }

    pub async fn list_decks(
        opts: ListOptions,
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let rqtype = opts.id.unwrap_or(0);
        if rqtype == 0 {
            let db = sessions_db.read().unwrap();
            match db.get(&session_id) {
                Some(user) => {
                    let c: Context = python! {
                        import psycopg2 as sq

                        con = sq.connect(
                            host="postgres",
                            database="wonki-db",
                            user="superuser",
                            password="verysecurepw"
                        )
                        cur = con.cursor()
                        cur.execute("SELECT deckname, views FROM decks WHERE owner = %s", ('user,))
                        res = cur.fetchall()
                        ls = []
                        views = []
                        for item in res:
                            ls.append(item[0])
                            views.append(item[1])
                        cur.close()
                        con.close()
                    };
                    let result = c.get::<Vec<String>>("ls");
                    let views_result = c.get::<Vec<i32>>("views");
                    let repl = Decklist {
                        names: result,
                        views: views_result,
                    };
                    Ok(warp::reply::json(&repl))
                }
                None => Err(warp::reject::not_found()),
            }
        } else {
            let target_user = opts.deck.unwrap();
            let db = sessions_db.read().unwrap();
            match db.get(&session_id) {
                Some(user) => {
                    let c: Context = python! {
                        import psycopg2 as sq

                        con = sq.connect(
                            host="postgres",
                            database="wonki-db",
                            user="superuser",
                            password="verysecurepw"
                        )
                        cur = con.cursor()
                        cur.execute("SELECT * FROM friends WHERE friendone = %s AND friendtwo = %s", ('user, 'target_user))
                        if cur.fetchone() is None:
                            fail = True
                        else:
                            cur.execute("SELECT deckname, views FROM decks WHERE owner = %s", ('target_user,))
                            res = cur.fetchall()
                            ls = []
                            views = []
                            for item in res:
                                ls.append(item[0])
                                views.append(item[1])
                            fail = False
                        cur.close()
                        con.close()
                    };
                    let fail = c.get::<bool>("fail");
                    if fail {
                        return Err(warp::reject::not_found());
                    } else {
                        let result = c.get::<Vec<String>>("ls");
                        let views_result = c.get::<Vec<i32>>("views");
                        let repl = Decklist {
                            names: result,
                            views: views_result,
                        };
                        Ok(warp::reply::json(&repl))
                    }
                }
                None => Err(warp::reject::not_found()),
            }
        }
    }
    pub fn cards_filter(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("cardlist")
            .and(warp::get())
            .and(warp::cookie("session_id"))
            .and(warp::query::<ListOptions>())
            .and(with_db(sessions_db))
            .and_then(list_cards)
    }

    pub async fn list_cards(
        session_id: String,
        opts: ListOptions,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let deck = opts.deck.unwrap();
        let db = sessions_db.read().unwrap();
        match db.get(&session_id) {
            Some(_) => {
                let c: Context = python! {
                    import psycopg2 as sq

                    con = sq.connect(
                        host="postgres",
                        database="wonki-db",
                        user="superuser",
                        password="verysecurepw"
                    )
                    cur = con.cursor()
                    cur.execute("SELECT id FROM cards WHERE deckname = %s", ('deck,))
                    res = cur.fetchall()
                    ls = []
                    for item in res:
                        ls.append(item[0])
                    cur.close()
                    con.close()
                };
                let result = c.get::<Vec<i32>>("ls");
                let repl = Cardlist { names: result };
                Ok(warp::reply::json(&repl))
            }
            None => Err(warp::reject::not_found()),
        }
    }

    pub fn share_filter(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("share")
            .and(warp::post())
            .and(json_body())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(share)
    }

    pub async fn share(
        body: Card,
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let deck = body.deck.unwrap();
        let target = body.content.unwrap();
        let db = sessions_db.read().unwrap();
        match db.get(&session_id) {
            Some(user) => {
                let clean_uname = clean(&target);
                let c: Context = python! {
                    import psycopg2 as sq

                    con = sq.connect(
                        host="postgres",
                        database="wonki-db",
                        user="superuser",
                        password="verysecurepw"
                    )
                    cur = con.cursor()
                    cur.execute("SELECT owner, time, cards FROM decks WHERE deckname = %s", ('deck,))
                    res = cur.fetchone()
                    if res is None:
                        owner = False
                    else:
                        if res[0] == 'user:
                            cur.execute("INSERT INTO decks (deckname, owner, cards, views, time) VALUES (%s,'"+ 'clean_uname +"', %s, 0, %s)", ('deck, res[2], res[1]))
                            con.commit()
                            owner = True
                        else:
                            owner = False
                    cur.close()
                    con.close()
                };
                let auth = c.get::<bool>("owner");
                if auth {
                    Ok(StatusCode::CREATED)
                } else {
                    Ok(StatusCode::UNAUTHORIZED)
                }
            }
            None => Ok(StatusCode::UNAUTHORIZED),
        }
    }

    pub fn pub_filter(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("pub")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and_then(get_pubkey)
    }

    pub async fn get_pubkey(opts: ListOptions) -> Result<impl warp::Reply, warp::Rejection> {
        let uname = opts.name.unwrap();
        let c: Context = python! {
            import base64
            import psycopg2 as sq

            con = sq.connect(
                host="postgres",
                database="wonki-db",
                user="superuser",
                password="verysecurepw"
            )
            cur = con.cursor()
            cur.execute("SELECT pub FROM users WHERE username = %s", ('uname, ))
            res = base64.b64encode(cur.fetchone()[0]).decode("ascii")
            cur.close()
            con.close()
        };
        let result = c.get::<String>("res");
        Ok(warp::reply::html(result))
    }

    pub fn token_filter(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("friendcode")
            .and(warp::get())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(get_token)
    }

    pub async fn get_token(
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let db = sessions_db.read().unwrap();
        match db.get(&session_id) {
            Some(user) => {
                let c: Context = python! {
                    from cryptography.hazmat.primitives import serialization
                    from cryptography.hazmat.primitives.asymmetric import ed25519
                    import jwt
                    import psycopg2 as sq

                    con = sq.connect(
                        host="postgres",
                        database="wonki-db",
                        user="superuser",
                        password="verysecurepw"
                    )
                    cur = con.cursor()
                    cur.execute("SELECT priv FROM users WHERE username = %s", ('user, ))
                    pr_key = bytes(cur.fetchone()[0])
                    if pr_key == b"None":
                        private_key = ed25519.Ed25519PrivateKey.generate()
                        pr_key = private_key.private_bytes(encoding=serialization.Encoding.PEM,format=serialization.PrivateFormat.PKCS8, encryption_algorithm=serialization.NoEncryption())
                        pub_key = private_key.public_key().public_bytes(encoding=serialization.Encoding.OpenSSH,format=serialization.PublicFormat.OpenSSH)
                        cur.execute("UPDATE users SET priv = %s, pub = %s WHERE username = %s", (pr_key, pub_key, 'user))
                        con.commit()
                    res = jwt.encode({"username": 'user}, pr_key, algorithm="EdDSA")
                    cur.close()
                    con.close()
                };
                let result = c.get::<String>("res");
                Ok(warp::reply::html(result).into_response())
            }
            None => Ok(StatusCode::UNAUTHORIZED.into_response()),
        }
    }

    pub fn friend_filter(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("friend")
            .and(warp::post())
            .and(json_body())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(add_friend)
    }

    pub async fn add_friend(
        body: Card,
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let token = body.content.unwrap();
        let target_name = body.deck.unwrap();
        let db = sessions_db.read().unwrap();
        match db.get(&session_id) {
            Some(user) => {
                let c: Context = python! {
                    import jwt
                    import psycopg2 as sq

                    con = sq.connect(
                        host="postgres",
                        database="wonki-db",
                        user="superuser",
                        password="verysecurepw"
                    )
                    cur = con.cursor()
                    cur.execute("SELECT pub FROM users WHERE username = %s", ('target_name, ))
                    pub_key = bytes(cur.fetchone()[0])
                    try:
                        decoded = jwt.decode('token, pub_key, algorithms=jwt.algorithms.get_default_algorithms())
                        decuser = decoded["username"]
                        if decoded["username"] == 'target_name:
                            cur.execute("INSERT INTO friends (friendone, friendtwo) VALUES (%s,%s)", ('user,'target_name))
                            cur.execute("INSERT INTO friends (friendone, friendtwo) VALUES (%s,%s)", ('target_name,'user))
                            con.commit()
                            fail = False
                        else:
                            fail = True
                    except:
                        fail = True
                    cur.close()
                    con.close()
                };
                let fail = c.get::<bool>("fail");
                if fail {
                    return Ok(StatusCode::UNAUTHORIZED);
                }
                Ok(StatusCode::CREATED)
            }
            None => Ok(StatusCode::UNAUTHORIZED),
        }
    }

    pub fn friendslist_filter(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("friendslist")
            .and(warp::get())
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(list_friends)
    }

    pub async fn list_friends(
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let db = sessions_db.read().unwrap();
        match db.get(&session_id) {
            Some(user) => {
                let c: Context = python! {
                    import psycopg2 as sq

                    con = sq.connect(
                        host="postgres",
                        database="wonki-db",
                        user="superuser",
                        password="verysecurepw"
                    )
                    cur = con.cursor()
                    cur.execute("SELECT friendtwo FROM friends WHERE friendone = %s", ('user,))
                    res = cur.fetchall()
                    ls = []
                    for item in res:
                        ls.append(item[0])
                    cur.close()
                    con.close()
                };
                let result = c.get::<Vec<String>>("ls");
                let repl = Decklist {
                    names: result,
                    views: vec![],
                };
                Ok(warp::reply::json(&repl))
            }
            None => Err(warp::reject::not_found()),
        }
    }

    fn json_body() -> impl Filter<Extract = (Card,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod models {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Card {
        pub id: Option<usize>,
        pub deck: Option<String>,
        pub content: Option<String>,
    }

    // The query parameters for cards
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub id: Option<i32>,
        pub deck: Option<String>,
        pub name: Option<String>,
        //pub content: Option<str>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Decklist {
        pub names: Vec<String>,
        pub views: Vec<i32>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Cardlist {
        pub names: Vec<i32>,
    }
}
mod users {
    use super::{hash, verify, User};
    use inline_python::{python, Context};

    use std::collections::HashMap;
    use std::convert::Infallible;
    use std::sync::{Arc, RwLock};
    use uuid::Uuid;
    use warp::http::StatusCode;
    use warp::Filter;

    pub fn options(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        login_filter(sessions_db.clone())
            .or(register_filter())
            .or(logout_filter(sessions_db))
    }

    pub fn with_db(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<
        Extract = (Arc<RwLock<HashMap<String, String>>>,),
        Error = std::convert::Infallible,
    > + Clone {
        warp::any().map(move || sessions_db.clone())
    }

    pub fn login_filter(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path("login"))
            .and(json_body())
            .and(with_db(sessions_db))
            .and_then(login)
    }

    pub fn register_filter(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path("register"))
            .and(json_body())
            .and_then(register)
    }

    pub fn logout_filter(
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path("logout")
            .and(warp::cookie("session_id"))
            .and(with_db(sessions_db))
            .and_then(logout)
    }
    async fn login(
        credentials: User,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let uname = credentials.username.clone();
        let c: Context = python! {
            import psycopg2 as sq

            con = sq.connect(
                host="postgres",
                database="wonki-db",
                user="superuser",
                password="verysecurepw"
            )

            cur = con.cursor()
            cur.execute("SELECT hash FROM users WHERE username = %s", ('uname,))
            user = cur.fetchone()
            if user is None:
                fail = True
            else:
                thash = user[0]
                fail = False
            cur.close()
            con.close()
        };
        let fail = c.get::<bool>("fail");
        if fail {
            let response = warp::reply::html("Unknown!\n");
            //let ncookie = format!("session_id={token}");
            let rpl = warp::reply::with_header(response, "", "");
            Ok(warp::reply::with_status(rpl, StatusCode::BAD_REQUEST))
        } else {
            let thash = c.get::<String>("thash");
            let matches = verify(&thash, &credentials.password);
            if matches {
                let mut users = sessions_db.write().unwrap();
                let token = Uuid::new_v4().to_string();
                users.insert(token.clone(), credentials.username.clone());
                let response = warp::reply::html("logged in!\n");
                let ncookie = format!("session_id={token}");
                let rpl = warp::reply::with_header(response, "Set-Cookie", ncookie);
                Ok(warp::reply::with_status(rpl, StatusCode::OK))
            } else {
                let response = warp::reply::html("Invalid username or password!\n");
                //let ncookie = format!("session_id={token}");
                let rpl = warp::reply::with_header(response, "", "");
                Ok(warp::reply::with_status(rpl, StatusCode::UNAUTHORIZED))
                //Ok(warp::reply::with_status("Invalid username or password", StatusCode::UNAUTHORIZED))
            }
        }
    }

    async fn register(
        new_user: User,
        //sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let phash = hash(&new_user.password);
        let uname = new_user.username;

        let c: Context = python! {
            import psycopg2 as sq
            import time

            con = sq.connect(
                host="postgres",
                database="wonki-db",
                user="superuser",
                password="verysecurepw"
            )

            cur = con.cursor()

            cur.execute("SELECT username FROM users WHERE username = %s", ('uname,))

            if cur.fetchone() is None:
                priv_val = "None"
                pub_key = "None"
                thistime = time.time()
                cur.execute("INSERT INTO users (username, hash, time, priv, pub) VALUES (%s, %s, %s, %s, %s)", ('uname, 'phash, thistime, priv_val, pub_key))
                con.commit()
                default_content = "None"
                cur.execute("INSERT INTO latest (username, content, time) VALUES (%s, %s, %s)", ('uname, default_content, thistime))
                con.commit()
                fail = False
            else:
                fail = True
            cur.close()
            con.close()
        };
        let hasfailed = c.get::<bool>("fail");
        if !hasfailed {
            return Ok(StatusCode::CREATED);
        } else {
            return Ok(StatusCode::CONFLICT);
        }
    }

    pub fn clean(input: &String) -> String {
        input
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect()
    }

    async fn logout(
        session_id: String,
        sessions_db: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        {
            let users = sessions_db.read().unwrap();
            match users.get(&session_id) {
                Some(user) => {
                    let _c: Context = python! {
                        import psycopg2 as sq

                        con = sq.connect(
                            host="postgres",
                            database="wonki-db",
                            user="superuser",
                            password="verysecurepw"
                        )

                        cur = con.cursor()
                        default_content = "None"
                        cur.execute("UPDATE latest SET content = %s WHERE username = %s", (default_content,'user))
                        con.commit()
                        cur.close()
                        con.close()
                    };
                }
                None => (),
            }
        }
        let mut users = sessions_db.write().unwrap();
        users.remove(&session_id);
        Ok(warp::redirect::found(warp::http::Uri::from_static("/")))
    }

    fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

async fn cleanup(sessions_db: Arc<RwLock<HashMap<String, String>>>) {
    //use std::panic::catch_unwind;
    //let dur = 900; //should be 15 Minutes in seconds
    println!("started cleanupscript");
    loop {
        let c: Context = python! {
            import psycopg2 as sq
            import time
            import string
            dur = 900
            limit = time.time() - dur

            con = sq.connect(
                host="postgres",
                database="wonki-db",
                user="superuser",
                password="verysecurepw"
            )

            cur = con.cursor()
            rvec = []

            cur.execute("SELECT username FROM users WHERE time < %s", (limit,))
            users = cur.fetchall()
            for user in users:
                cur.execute("DELETE FROM friends WHERE friendone = %s", (user[0],))
                cur.execute("DELETE FROM friends WHERE friendtwo = %s", (user[0],))
                rvec.append(user[0])

            cur.execute("DELETE FROM users WHERE time < %s", (limit,))
            cur.execute("DELETE FROM latest WHERE time < %s", (limit,))
            cur.execute("DELETE FROM decks WHERE time < %s", (limit,))
            cur.execute("DELETE FROM cards WHERE time < %s", (limit,))
            con.commit()

            cur.close()
            con.close()
        };
        let uservec = c.get::<Vec<String>>("rvec");
        /*
        for val in uservec.clone() {
            println!("removed user {:#?}", val);
        }
        */
        {
            let mut users = sessions_db.write().unwrap();
            users.retain(|_key, val| !uservec.contains(&val));
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
#[tokio::main]
async fn main() {
    let _c: Context = python! {
        import psycopg2 as sq

        con = sq.connect(
            host="postgres",
            database="wonki-db",
            user="superuser",
            password="verysecurepw"
        )

        cur = con.cursor()

        cur.execute("SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name='users')")
        if not cur.fetchone()[0]:
            cur.execute("""
                CREATE TABLE users (
                    username TEXT PRIMARY KEY,
                    hash TEXT,
                    time INTEGER,
                    priv BYTEA,
                    pub BYTEA
                )
            """)

        cur.execute("SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name='latest')")
        if not cur.fetchone()[0]:
            cur.execute("""
                CREATE TABLE latest (
                    username TEXT PRIMARY KEY,
                    content TEXT,
                    time INTEGER
                )
            """)
        
        cur.execute("SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name='decks')")
        if not cur.fetchone()[0]:
            cur.execute("""
                CREATE TABLE decks (
                    deckname TEXT,
                    owner TEXT,
                    cards INTEGER,
                    views INTEGER,
                    time INTEGER
                )
            """)
        
        cur.execute("SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name='cards')")
        if not cur.fetchone()[0]:
            cur.execute("""
                CREATE TABLE cards (
                    id INTEGER,
                    deckname TEXT,
                    content TEXT,
                    time INTEGER
                )
            """)
        
        cur.execute("SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name='friends')")
        if not cur.fetchone()[0]:
            cur.execute("""
                CREATE TABLE friends (
                    friendone TEXT,
                    friendtwo TEXT
                )
            """)
        con.commit()
        cur.close()
        con.close()
    };

    let sessions_db = Arc::new(RwLock::new(HashMap::<String, String>::new())); // session_id -> username
    let api = pages::options()
        .or(users::options(sessions_db.clone()))
        .or(cards::options(sessions_db.clone()));

    tokio::spawn(async { cleanup(sessions_db).await });

    warp::serve(api).run(([0, 0, 0, 0], 2027)).await;
}
