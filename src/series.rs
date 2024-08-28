use rusqlite::{Connection, Error};
use uuid::Uuid;

use crate::db::get_connection;

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
        name: String,
        is_finished: Option<bool>,
        is_airing_finished: Option<bool>,
        total_episodes: Option<i32>,
        current_episode: Option<i32>,
    ) -> Self {
        let guid = Uuid::new_v4().to_string();
        let current_episode = current_episode.unwrap_or(0);
        let total_episodes = total_episodes.unwrap_or(0);
        let is_airing_finished = is_airing_finished.unwrap_or(false);

        // debug_assert!(!name.is_empty(), "Name must not be empty");
        // debug_assert!(
        //     current_episode >= 0,
        //     "Current episode must be a positive number"
        // );
        // debug_assert!(
        //     total_episodes >= 0,
        //     "Total episodes must be a positive number"
        // );
        // debug_assert!(
        //     current_episode > total_episodes,
        //     "Current episode cannot be higher than total"
        // );

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

    pub(crate) fn get(take: usize, skip: usize) -> Result<Vec<Self>, Error> {
        let ctx = get_connection()?;
        let mut series = Vec::new();

        let mut stmt = ctx.prepare(
            r"
            SELECT guid, name, is_finished, is_airing_finished, total_episodes, current_episode
              FROM series
            ORDER BY name
             LIMIT ?
            OFFSET ?
            ",
        )?;
        let mut rows = stmt.query([take, skip])?;

        while let Some(row) = rows.next()? {
            series.push(Self {
                guid: row.get(0)?,
                name: row.get(1)?,
                is_finished: row.get(2)?,
                is_airing_finished: row.get(3)?,
                total_episodes: row.get(4)?,
                current_episode: row.get(5)?,
            });
        }

        Ok(series)
    }

    pub(crate) fn get_by_guid(guid: String) -> Result<Option<Self>, Error> {
        let ctx = get_connection()?;
        let mut stmt = ctx.prepare(
            r#"
            SELECT guid, name, is_finished, is_airing_finished, total_episodes, current_episode
            FROM series
            WHERE guid = ?
            "#,
        )?;
        let mut rows = stmt.query([&guid])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self {
                guid: row.get(0)?,
                name: row.get(1)?,
                is_finished: row.get(2)?,
                is_airing_finished: row.get(3)?,
                total_episodes: row.get(4)?,
                current_episode: row.get(5)?,
            })),
            None => Ok(None),
        }
    }

    pub(crate) fn try_insert(&mut self) -> Result<bool, Error> {
        let ctx = get_connection()?;
        let mut stmt = ctx.prepare(
            r#"
            INSERT INTO series (guid, name, is_finished, is_airing_finished, total_episodes, current_episode)
            VALUES (?, ?, ?, ?, ?, ?)
            "#)?;

        stmt.execute([
            &self.guid,
            &self.name,
            &(if self.is_finished { "1" } else { "0" }).to_string(),
            &(if self.is_airing_finished { "1" } else { "0" }).to_string(),
            &self.total_episodes.to_string(),
            &self.current_episode.to_string(),
        ])?;

        Ok(true)
    }

    pub(crate) fn try_update(&mut self) -> Result<bool, Error> {
        let ctx = get_connection()?;
        let mut stmt = ctx.prepare(
            r#"
            UPDATE series
            SET name = ?, is_finished = ?, is_airing_finished = ?, total_episodes = ?, current_episode = ?
            WHERE guid = ?
            "#)?;
        stmt.execute([
            &self.name,
            &(if self.is_finished { "1" } else { "0" }).to_string(),
            &(if self.is_airing_finished { "1" } else { "0" }).to_string(),
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

    pub(crate) fn count_total() -> Result<usize, Error> {
        let ctx = get_connection()?;
        ctx.query_row("SELECT COUNT(guid) FROM series", [], |row| row.get(0))
    }
}
