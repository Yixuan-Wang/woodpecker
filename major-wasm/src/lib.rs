use js_sys::SyntaxError;
use major::hole::{
    reply::{RawReplyPage, ReplyEntry},
    HoleEntry, RawHolePage,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_DEFINITION: &'static str = r#"
export interface HoleKindText {
    type: 'text';
}

export interface HoleKindImage {
    type: 'image';
    url: string;
}

export interface HoleKindAudio {
    type: 'audio';
    url: string;
}

export type HoleKind = HoleKindText | HoleKindImage | HoleKindAudio

export interface Hole {
    id: number;
    text: string;
    kind: HoleKind;
    timestamp: string;
    reply: number;
    likenum: number;
    tag: string | null;
}

export interface HoleEntry {
    entry: Hole;
    snapshot: string;
}

export interface Reply {
    id: number;
    hole: number;
    name: string;
    text: string;
    dz: boolean;
    timestamp: string;
    tag: string | null;
}

export interface ReplyEntry {
    entry: Reply;
    snapshot: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "[HoleEntry]")]
    pub type JsArrayHoleEntry;
    #[wasm_bindgen(typescript_type = "[ReplyEntry]")]
    pub type JsArrayReplyEntry;
}

#[wasm_bindgen]
pub fn parse_holes(input: &str) -> Result<JsArrayHoleEntry, JsValue> {
    parse::<Vec<HoleEntry>, JsArrayHoleEntry, HoleEntry>(input)
}

#[wasm_bindgen]
pub fn parse_holes_from_api(input: &str) -> Result<JsArrayHoleEntry, JsValue> {
    parse::<RawHolePage, JsArrayHoleEntry, HoleEntry>(input)
}

#[wasm_bindgen]
pub fn parse_replies(input: &str) -> Result<JsArrayReplyEntry, JsValue> {
    parse::<Vec<ReplyEntry>, JsArrayReplyEntry, ReplyEntry>(input)
}

#[wasm_bindgen]
pub fn parse_replies_from_api(input: &str) -> Result<JsArrayReplyEntry, JsValue> {
    parse::<RawReplyPage, JsArrayReplyEntry, ReplyEntry>(input)
}

fn parse<'a, I, O, E>(input: &'a str) -> Result<O, JsValue>
where
    I: Deserialize<'a> + IntoIterator<Item = E>,
    O: JsCast,
    E: Serialize,
{
    let x = serde_json::from_str::<I>(input);
    match x {
        Ok(r) => {
            let v = r.into_iter().collect::<Vec<_>>();
            Ok(JsValue::from_serde(&v)
                .map_err(|err| SyntaxError::new(&err.to_string()))?
                .unchecked_into::<O>())
        }
        Err(err) => Err(SyntaxError::new(&err.to_string()).into()),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        /* let mut input = String::default();
        file.read_to_string(&mut input).unwrap();
        let x: BTreeSet<HoleEntry> = serde_json::from_str(&input).unwrap();
        dbg!(x); */
    }
}
