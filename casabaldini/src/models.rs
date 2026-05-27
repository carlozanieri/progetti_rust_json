use serde::{Serialize, Deserialize};
use reqwest::Client;
// ==========================
// CONFIG API
// ==========================
const API_BASE: &str = "https://json.casabaldini.eu/api/v1";
const API_MENU: &str = "https://json.casabaldini.eu/api/v1/menu";
const API_SUBMENU: &str = "https://json.casabaldini.eu/api/v1/menu";
const API_SLIDER: &str = "https://json.casabaldini.eu/api/v1/slider";
const API_LINKS: &str = "https://json.casabaldini.eu/api/v1/links";
const API_FOODS: &str = "https://json.casabaldini.eu/api/v1/foods";
// STRUCTS (INVARIATE)
// ==========================

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Menus {
    pub id: i64,
    pub codice: String,
    pub radice: String,
    pub livello: i64,
    pub titolo: String,
    pub link: String,
    pub ordine: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Slider {
    pub id: i64,
    pub img: String,
    pub titolo: String,
    pub testo: String,
    pub caption: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    pub id: i64,
    pub codice: String,
    pub img: String,
    pub titolo: String,
    pub descrizione: String,
    pub link: String,
    pub height: String,
    pub width: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Foods {
    pub id: i64,
    pub codice: String,
    pub img: String,
    pub titolo: String,
    pub descrizione: String,
    pub link: String,
    pub width: String,
    pub height: String,
    pub indirizzo: String,
    pub telefono: String,
    pub apiedi: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MenuItem {
    pub parent: Menus,
    pub children: Vec<Menus>,
}
// ==========================
// FUNZIONI API (CLIENT)
// ==========================

// -------- MENU --------
pub async fn get_menu() -> Result<Vec<MenuItem>, String> {
    let res = reqwest::get(API_MENU)
        .await
        .map_err(|e| e.to_string())?;

    let text = res.text().await.map_err(|e| e.to_string())?;

    println!("MENU RAW: {}", text);

    serde_json::from_str::<Vec<MenuItem>>(&text)
        .map_err(|e| e.to_string())
}

// -------- SUBMENU --------
pub async fn get_slide(dir: String) -> Result<Vec<Slider>, String> {
    //let url = format!("{}/sliders?dir={}", API_SLIDER, dir);
    let url = format!("{}/sliders?dir={}", API_SLIDER, dir);
    let res = reqwest::get(API_SLIDER)
        .await
        .map_err(|e| e.to_string())?;
    
    let text = res.text().await.map_err(|e| e.to_string())?;

    //println!("MENU RAW: {}", text);
    
    serde_json::from_str::<Vec<Slider>>(&text)
        .map_err(|e| e.to_string())
}

// -------- SLIDER --------
pub async fn get_sliders(dir: String) -> Result<Vec<Slider>, reqwest::Error> {
    let url = format!("{}/sliders?dir={}", API_SLIDER, dir);

    reqwest::get(url)
        .await?
        .json::<Vec<Slider>>()
        .await
}

// -------- IMMAGINI BASE64 --------
pub async fn get_single_image_b64(name: String, dir: String,) -> Result<String, reqwest::Error> {
    let url = format!("{}assets/img/?dir={}&name={}", API_SLIDER, dir,name );
    let path = format!("{}assets/img/{}/{}",API_SLIDER, dir, name);
    reqwest::get(path)
        .await?
        .text()
        .await
}

pub async fn get_single_img_64(name: String,dir: String) -> Result<Vec<Slider>, String> {
    //let url = format!("{}/sliders?dir={}", API_SLIDER, dir);
    let url = format!("{}/sliders?dir={}", API_SLIDER, dir);
    let path = format!("assets/img/{}/{}",dir, name);
    let res = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?;
    
    let text = res.text().await.map_err(|e| e.to_string())?;

    //println!("MENU RAW: {}", text);
    
    serde_json::from_str::<Vec<Slider>>(&text)
        .map_err(|e| e.to_string())
}

// -------- LINKS --------
pub async fn get_links() -> Result<Vec<Links>, reqwest::Error> {
    reqwest::get(format!("{}/links", API_LINKS))
        .await?
        .json::<Vec<Links>>()
        .await
}


pub async fn get_food() -> Result<Vec<Foods>, String> {
    //let url = format!("{}/foods", API_FOODS);
    let url = format!("{}/foods", API_FOODS);
    let res = reqwest::get(API_FOODS)
        .await
        .map_err(|e| e.to_string())?;
    
    let text = res.text().await.map_err(|e| e.to_string())?;

    
    serde_json::from_str::<Vec<Foods>>(&text)
        .map_err(|e| e.to_string())
}