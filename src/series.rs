use std::io::stdout;

use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use rusqlite::{Connection, Error};

use crate::db::DbContext;

#[derive(Debug, Clone)]
pub(crate) struct Series {
    pub(crate) guid: String,
    pub(crate) name: String,
    pub(crate) is_finished: bool,
    pub(crate) is_airing_finished: bool,
    pub(crate) total_episodes: i32,
    pub(crate) current_episode: i32,
}

impl Series {
    pub(crate) fn new(
        guid: String,
        name: String,
        is_finished: Option<bool>,
        is_airing_finished: Option<bool>,
        total_episodes: Option<i32>,
        current_episode: Option<i32>,
    ) -> Self {
        let current_episode = current_episode.unwrap_or(0);
        let total_episodes = total_episodes.unwrap_or(0);
        let is_airing_finished = is_airing_finished.unwrap_or(false);

        assert!(guid.len() == 36, "GUID must be 36 characters long");
        assert!(!name.is_empty(), "Name must not be empty");
        assert!(
            current_episode >= 0,
            "Current episode must be a positive number"
        );
        assert!(
            total_episodes >= 0,
            "Total episodes must be a positive number"
        );

        let is_finished =
            is_finished.unwrap_or(is_airing_finished && current_episode == total_episodes);

        Self {
            guid,
            name,
            is_finished,
            is_airing_finished,
            total_episodes,
            current_episode,
        }
    }

    pub(crate) fn get(
        take: Option<usize>,
        skip: Option<usize>,
    ) -> Result<Vec<Self>, rusqlite::Error> {
        let ctx = DbContext::new()?;
        let mut series = Vec::new();

        let mut stmt = ctx.prepare(
            r"
            SELECT guid, name, isfinished, isairingfinished, totalepisodes, currentepisode
              FROM series
            ORDER BY name
             LIMIT ?
            OFFSET ?
            ",
        )?;
        let mut rows = stmt.query([take.unwrap_or(9_999), skip.unwrap_or(0)])?;

        while let Some(row) = rows.next()? {
            series.push(Self::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ));
        }

        Ok(series)
    }

    pub(crate) fn get_by_guid(guid: String, conn: &Connection) -> Result<Option<Self>, Error> {
        let mut stmt = conn.prepare("SELECT guid, name, isfinished, isairingfinished, totalepisodes, currentepisode FROM series WHERE guid = ?")?;
        let mut rows = stmt.query([&guid])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))),
            None => Ok(None),
        }
    }

    pub(crate) fn try_insert(&mut self, conn: &Connection) -> Result<bool, Error> {
        let mut stmt = conn.prepare("INSERT INTO series (guid, name, isfinished, isairingfinished, totalepisodes, currentepisode) VALUES (?, ?, ?, ?, ?, ?)")?;
        stmt.execute([
            &self.guid,
            &self.name,
            &self.is_finished.to_string(),
            &self.is_airing_finished.to_string(),
            &self.total_episodes.to_string(),
            &self.current_episode.to_string(),
        ])?;

        Ok(true)
    }

    pub(crate) fn try_update(&mut self, conn: &Connection) -> Result<bool, Error> {
        let mut stmt = conn.prepare("UPDATE series SET name = ?, isfinished = ?, isairingfinished = ?, totalepisodes = ?, currentepisode = ? WHERE guid = ?")?;
        stmt.execute([
            &self.name,
            &self.is_finished.to_string(),
            &self.is_airing_finished.to_string(),
            &self.total_episodes.to_string(),
            &self.current_episode.to_string(),
            &self.guid,
        ])?;

        Ok(true)
    }

    pub(crate) fn try_delete(&mut self, conn: &Connection) -> Result<bool, Error> {
        let mut stmt = conn.prepare("DELETE FROM series WHERE guid = ?")?;
        stmt.execute([&self.guid])?;

        Ok(true)
    }
}
