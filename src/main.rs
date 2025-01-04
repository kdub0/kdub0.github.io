use std::{
    collections::HashMap,
    io::{BufReader, Write},
};

use log::info;
use serde::{Deserialize, Serialize};
use simplelog::{Config, TermLogger};

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Link {
    Arxiv(String),
    Pdf(String),
    Url(UrlLink),
}

#[derive(Serialize, Deserialize, Debug)]
struct UrlLink {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Publication {
    InProceedings(InProceedings),
    JournalArticle(JournalArticle),
    MastersThesis(MastersThesis),
    TechnicalReport(TechnicalReport),
}

#[derive(Serialize, Deserialize, Debug)]
struct InProceedings {
    authors: Vec<String>,
    title: String,
    booktitle: String,
    pages: Option<String>,
    year: u32,
    note: Option<String>,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JournalArticle {
    authors: Vec<String>,
    title: String,
    journal: String,
    year: u32,
    volume: Option<i32>,
    number: Option<i32>,
    month: Option<String>,
    pages: Option<String>,
    note: Option<String>,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MastersThesis {
    authors: Vec<String>,
    title: String,
    school: String,
    year: u32,
    note: Option<String>,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TechnicalReport {
    authors: Vec<String>,
    title: String,
    institution: String,
    number: String,
    year: u32,
    note: Option<String>,
    links: Vec<Link>,
}

trait HasYear {
    fn get_year(&self) -> u32;
}

impl HasYear for InProceedings {
    fn get_year(&self) -> u32 {
        self.year
    }
}

impl HasYear for JournalArticle {
    fn get_year(&self) -> u32 {
        self.year
    }
}

impl HasYear for MastersThesis {
    fn get_year(&self) -> u32 {
        self.year
    }
}

impl HasYear for TechnicalReport {
    fn get_year(&self) -> u32 {
        self.year
    }
}

impl HasYear for Publication {
    fn get_year(&self) -> u32 {
        match self {
            Publication::InProceedings(t) => t.year,
            Publication::JournalArticle(t) => t.year,
            Publication::MastersThesis(t) => t.year,
            Publication::TechnicalReport(t) => t.year,
        }
    }
}

fn read_authors() -> anyhow::Result<HashMap<String, Author>> {
    info!("Reading authors.yaml");
    let file = std::fs::File::open("authors.yaml")?;
    let mut author_map = HashMap::new();
    for author in serde_yaml::from_reader::<_, Vec<Author>>(BufReader::new(file))? {
        if author_map.contains_key(&author.name) {
            anyhow::bail!("Duplicate author named {}", author.name);
        } else {
            author_map.insert(author.name.clone(), author);
        }
    }
    Ok(author_map)
}

fn read_publications() -> anyhow::Result<Vec<Publication>> {
    info!("Reading publications.yaml");
    let file = std::fs::File::open("publications.yaml")?;
    Ok(serde_yaml::from_reader::<_, Vec<Publication>>(
        BufReader::new(file),
    )?)
}

fn write_head(writer: &mut impl Write) -> anyhow::Result<()> {
    writeln!(
        writer,
        r#"<!DOCTYPE html>
<html lang="en">"#
    )?;
    Ok(())
}

fn write_tail(writer: &mut impl Write) -> anyhow::Result<()> {
    writeln!(
        writer,
        r#"      </ul>
    </div>
  </body>
</html>"#
    )?;
    Ok(())
}

fn write_header(writer: &mut impl Write) -> anyhow::Result<()> {
    writeln!(
        writer,
        r#"  <head>
    <title>Kevin Waugh's Publications</title>
    <link rel="shortcut icon" href="favicon.png" type="image/x-icon">
    <meta charset="utf-8">
    <link href="https://fonts.googleapis.com/css?family=Lato" rel="stylesheet">
    <link rel="stylesheet" type="text/css" href="publications.css">
  </head>
  <body>
    <div class="content">"#
    )?;
    Ok(())
}

fn write_navbar(writer: &mut impl Write, years: &[u32]) -> anyhow::Result<()> {
    writeln!(writer, r#"      <ul class="navbar">"#)?;
    let mut first = true;
    for year in years.iter().cloned() {
        if first {
            first = false;
            writeln!(writer, r##"        <li><a href="#{year}">{year}</a></li>"##)?;
        } else {
            writeln!(
                writer,
                r##"        <li>| <a href="#{year}">{year}</a></li>"##
            )?;
        }
    }
    writeln!(
        writer,
        r##"        <li>| <a href="https://scholar.google.com/citations?hl=en&user=l5ryKEkAAAAJ">Scholar</a></li>"##
    )?;
    writeln!(writer, r#"      </ul>"#)?;
    writeln!(writer, r#"      <ul class="entries">"#)?;
    Ok(())
}

fn write_year_header(writer: &mut impl Write, year: u32) -> anyhow::Result<()> {
    writeln!(
        writer,
        r#"        <h2 class="year" id="{year}">{year}</h2>"#
    )?;
    Ok(())
}

fn write_author(
    writer: &mut impl Write,
    author_map: &HashMap<String, Author>,
    author: &str,
) -> anyhow::Result<()> {
    match author_map.get(author) {
        Some(auth) => write!(
            writer,
            r#"<a class="author_link" href="{}"><span class="author">{}</span></a>"#,
            auth.url, author
        ),
        _ => write!(writer, r#"<span class="author">{}</span>"#, author),
    }?;
    Ok(())
}

fn write_authors(
    writer: &mut impl Write,
    author_map: &HashMap<String, Author>,
    authors: &[String],
) -> anyhow::Result<()> {
    for (i, author) in authors.iter().enumerate() {
        if i + 1 == authors.len() {
            if authors.len() == 1 {
            } else if authors.len() == 2 {
                write!(writer, " and ")?;
            } else {
                write!(writer, ", and ")?;
            }
        } else if i > 0 {
            write!(writer, ", ")?;
        }
        write_author(writer, author_map, author)?;
    }
    write!(writer, ". ")?;
    Ok(())
}

fn write_title(writer: &mut impl Write, title: &str) -> anyhow::Result<()> {
    write!(writer, r#"<span class="title">{title}</span>. "#)?;
    Ok(())
}

fn write_location(writer: &mut impl Write, location: &str) -> anyhow::Result<()> {
    write!(writer, r#"<span class="location">{location}</span>."#)?;
    Ok(())
}

fn write_note(writer: &mut impl Write, note: &Option<String>) -> anyhow::Result<()> {
    match note {
        Some(st) => write!(writer, r#" <span class="note">{st}</span>."#)?,
        _ => (),
    }
    Ok(())
}

fn write_links(writer: &mut impl Write, links: &[Link]) -> anyhow::Result<()> {
    if !links.is_empty() {
        write!(writer, r#"<ul class="links">"#)?;
        for link in links {
            write!(writer, r#"<li class="link_item">"#)?;
            match link {
                Link::Arxiv(id) => write!(
                    writer,
                    r#"<a class="link" href="https://arxiv.org/abs/{id}">[arXiv]</a>"#
                )?,
                Link::Pdf(url) => write!(writer, r#"<a class="link" href="{url}">[PDF]</a>"#)?,
                Link::Url(link) => write!(
                    writer,
                    r#"<a class="link" href="{}">[{}]</a>"#,
                    link.url, link.name
                )?,
            }
            write!(writer, r#"</li>"#)?;
        }
        write!(writer, r#"</ul>"#)?;
    }
    Ok(())
}

fn make_pages(pages: &Option<String>) -> String {
    match pages {
        Some(pages) => {
            let mut out: String = " ".into();
            let mut first = true;
            for p in pages.splitn(2, '-') {
                if first {
                    out.push_str(p);
                    first = false;
                } else {
                    out.push_str("&ndash;");
                    out.push_str(p);
                }
            }
            out.push(',');
            out
        }
        _ => "".into(),
    }
}

fn write_in_proceedings(
    writer: &mut impl Write,
    author_map: &HashMap<String, Author>,
    publ: &InProceedings,
) -> anyhow::Result<()> {
    write_authors(writer, author_map, &publ.authors)?;
    write_title(writer, &publ.title)?;
    write_location(
        writer,
        &format!(
            "In proceedings of {},{} {}",
            publ.booktitle,
            make_pages(&publ.pages),
            publ.year
        ),
    )?;
    write_note(writer, &publ.note)?;
    write_links(writer, &publ.links)?;
    Ok(())
}

fn write_journal_article(
    writer: &mut impl Write,
    author_map: &HashMap<String, Author>,
    publ: &JournalArticle,
) -> anyhow::Result<()> {
    write_authors(writer, author_map, &publ.authors)?;
    write_title(writer, &publ.title)?;

    let mut loc = format!("In {}", publ.journal);
    {
        use std::fmt::Write;
        if let Some(volume) = publ.volume {
            write!(&mut loc, " {volume}")?;
            if let Some(number) = publ.number {
                write!(&mut loc, "({number})")?;
            }
        }

        loc.push_str(", ");
        if publ.pages.is_some() {
            loc.push_str(&make_pages(&publ.pages));
        }

        if let Some(month) = &publ.month {
            loc.push_str(&month);
            loc.push(' ');
        }
        write!(&mut loc, "{}", publ.year)?;
    }

    write_location(writer, &loc)?;

    write_note(writer, &publ.note)?;
    write_links(writer, &publ.links)?;
    Ok(())
}

fn write_masters_thesis(
    writer: &mut impl Write,
    author_map: &HashMap<String, Author>,
    publ: &MastersThesis,
) -> anyhow::Result<()> {
    write_authors(writer, author_map, &publ.authors)?;
    write_title(writer, &publ.title)?;
    write_location(
        writer,
        &format!("Master's thesis, {}, {}", publ.school, publ.year),
    )?;
    write_note(writer, &publ.note)?;
    write_links(writer, &publ.links)?;
    Ok(())
}

fn write_technical_report(
    writer: &mut impl Write,
    author_map: &HashMap<String, Author>,
    publ: &TechnicalReport,
) -> anyhow::Result<()> {
    write_authors(writer, author_map, &publ.authors)?;
    write_title(writer, &publ.title)?;
    write_location(
        writer,
        &format!(
            "Technical report {}, {}, {}",
            publ.number, publ.institution, publ.year
        ),
    )?;
    write_note(writer, &publ.note)?;
    write_links(writer, &publ.links)?;
    Ok(())
}

fn write_publication(
    writer: &mut impl Write,
    author_map: &HashMap<String, Author>,
    publ: &Publication,
) -> anyhow::Result<()> {
    write!(writer, r#"        <li class="entry">"#)?;
    match publ {
        Publication::InProceedings(t) => write_in_proceedings(writer, author_map, t),
        Publication::JournalArticle(t) => write_journal_article(writer, author_map, t),
        Publication::MastersThesis(t) => write_masters_thesis(writer, author_map, t),
        Publication::TechnicalReport(t) => write_technical_report(writer, author_map, t),
    }?;
    writeln!(writer, r#"</li>"#)?;
    Ok(())
}

fn write_publications(
    writer: &mut impl Write,
    author_map: &HashMap<String, Author>,
    pubs: &[Publication],
) -> anyhow::Result<()> {
    for pubs_by_year in pubs.chunk_by(|p1, p2| p1.get_year() == p2.get_year()) {
        let year = pubs_by_year[0].get_year();
        info!("Processing publications from {year}");
        write_year_header(writer, year)?;
        for publ in pubs_by_year {
            write_publication(writer, author_map, publ)?;
        }
        info!("{} publications from {year}", pubs.len());
    }
    info!("Wrote {} total publications", pubs.len());
    Ok(())
}

fn main() {
    TermLogger::init(
        simplelog::LevelFilter::Info,
        Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let author_map = read_authors().unwrap();
    info!("Read {} authors", author_map.len());

    let mut publications = read_publications().unwrap();
    info!("Read {} publications", publications.len());

    info!("Opening index.html");
    let file = std::fs::File::create("github-pages/index.html").unwrap();
    let mut writer = std::io::BufWriter::new(file);

    info!("Writing head");
    write_head(&mut writer).unwrap();

    info!("Writing header");
    write_header(&mut writer).unwrap();

    // Sort by year in reverse, otherwise in order of publications.yaml.
    publications.sort_by(|p1, p2| p2.get_year().cmp(&p1.get_year()));

    info!("Writing navbar");
    write_navbar(
        &mut writer,
        publications
            .as_slice()
            .chunk_by(|p1, p2| p1.get_year() == p2.get_year())
            .into_iter()
            .map(|pubs| pubs[0].get_year())
            .collect::<Vec<u32>>()
            .as_slice(),
    )
    .unwrap();

    info!("Writing publications");
    write_publications(&mut writer, &author_map, publications.as_slice()).unwrap();

    info!("Writing tail");
    write_tail(&mut writer).unwrap();

    info!("Done writing index.html");
}
