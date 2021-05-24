use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::BufReader;

//type XsdString = String;
type XsdNonNegativeInteger = u64;
type XsdGYear = String;
type XsdDateTime = String;
type XsdDecimal = f64;
type XsdAnyUri = String;

#[derive(Serialize, Deserialize)]
struct Gpx {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "@creator")]
    creator: String,

    metadata: Option<Metadata>,
    #[serde(default)]
    wpt: Vec<Wpt>,
    #[serde(default)]
    rte: Vec<Rte>,
    #[serde(default)]
    trk: Vec<Trk>,
    extensions: Option<Extensions>,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: Option<String>,
    desc: Option<String>,
    author: Option<Person>,
    copyright: Option<Copyright>,
    #[serde(default)]
    link: Vec<Link>,
    time: Option<XsdDateTime>,
    keywords: Option<String>,
    bounds: Option<Bounds>,
    extensions: Option<Extensions>,
}

#[derive(Serialize, Deserialize)]
struct Wpt {
    #[serde(rename = "@lat")]
    lat: Latitude,
    #[serde(rename = "@lon")]
    lon: Longitude,

    ele: Option<XsdDecimal>,
    time: Option<XsdDateTime>,
    magvar: Option<Degrees>,
    geoidheight: Option<XsdDecimal>,
    name: Option<String>,
    cmt: Option<String>,
    desc: Option<String>,
    src: Option<String>,
    #[serde(default)]
    link: Vec<Link>,
    sym: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
    fix: Option<Fix>,
    sat: Option<XsdNonNegativeInteger>,
    hdop: Option<XsdDecimal>,
    vdop: Option<XsdDecimal>,
    pdop: Option<XsdDecimal>,
    ageofdgpsdata: Option<XsdDecimal>,
    dgpsid: Option<XsdDecimal>,
    extensions: Option<Extensions>,
}

#[derive(Serialize, Deserialize)]
struct Rte {
    name: Option<String>,
    cmt: Option<String>,
    desc: Option<String>,
    src: Option<String>,
    #[serde(default)]
    link: Vec<Link>,
    number: Option<XsdNonNegativeInteger>,
    #[serde(rename = "type")]
    type_: Option<String>,
    extensions: Option<Extensions>,

    rtept: Vec<Wpt>,
}

#[derive(Serialize, Deserialize)]
struct Trk {
    name: Option<String>,
    cmt: Option<String>,
    desc: Option<String>,
    src: Option<String>,
    #[serde(default)]
    link: Vec<Link>,
    number: Option<XsdNonNegativeInteger>,
    #[serde(rename = "type")]
    type_: Option<String>,
    extensions: Option<Extensions>,

    trkseg: Vec<TrkSeg>,
}

#[derive(Serialize, Deserialize)]
struct Extensions {
    // ???
}

#[derive(Serialize, Deserialize)]
struct TrkSeg {
    trkpt: Vec<Wpt>,
    extensions: Option<Extensions>,
}

#[derive(Serialize, Deserialize)]
struct Copyright {
    #[serde(rename = "@author")]
    author: String,

    year: Option<XsdGYear>,
    license: Option<XsdAnyUri>,
}

#[derive(Serialize, Deserialize)]
struct Link {
    #[serde(rename = "@href")]
    href: XsdAnyUri,

    text: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Email {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@domain")]
    domain: String,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: Option<String>,
    email: Option<Email>,
    link: Option<Link>,
}

#[derive(Serialize, Deserialize)]
struct Pt {
    #[serde(rename = "@lat")]
    lat: Latitude,
    #[serde(rename = "@lon")]
    lon: Longitude,

    ele: Option<XsdDecimal>,
    time: Option<XsdDateTime>,
}

#[derive(Serialize, Deserialize)]
struct PtSeg {
    pt: Vec<Pt>,
}

#[derive(Serialize, Deserialize)]
struct Bounds {
    #[serde(rename = "@minlat")]
    minlat: Latitude,
    #[serde(rename = "@minlon")]
    minlon: Longitude,
    #[serde(rename = "@maxlat")]
    maxlat: Latitude,
    #[serde(rename = "@maxlon")]
    maxlon: Longitude,
}

#[derive(Serialize, Deserialize)]
struct Latitude(f64);

#[derive(Serialize, Deserialize)]
struct Longitude(f64);

#[derive(Serialize, Deserialize)]
struct Degrees(f64);

#[derive(Serialize, Deserialize)]
enum Fix {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "2d")]
    _2d,
    #[serde(rename = "3d")]
    _3d,
    #[serde(rename = "dgps")]
    Dgps,
    #[serde(rename = "pps")]
    Pps,
}

#[derive(Serialize, Deserialize)]
struct DgpsStation(u16);

fn main() {
    let arg = env::args().nth(1).expect("missing gpx argument");

    let reader = BufReader::new(fs::File::open(arg).unwrap());
    let gpx: Gpx = match xserde::from_reader(reader) {
        Err(err) => {
            println!("error: {}", err);
            return;
        }
        Ok(gpx) => gpx,
    };
    println!("Version: {}", gpx.version);
    println!("Creator: {}", gpx.creator);
    if let Some(metadata) = gpx.metadata {
        if let Some(name) = metadata.name {
            println!("Name: {}", &name);
        }
        if let Some(desc) = metadata.desc {
            println!("Description: {}", &desc);
        }
    }

    for trk in gpx.trk {
        println!(
            "Track: {}",
            &trk.name.as_ref().map(|x| x as &str).unwrap_or("<Unnamed>")
        );
        for (i, seg) in trk.trkseg.iter().enumerate() {
            println!("  Segment {}: {} points", i, seg.trkpt.len());
        }
    }
}
