use chrono::DateTime;
use git2::{Error, Repository};

pub struct GitStruct {
    repo: Repository,
}

impl GitStruct {
    pub fn new(path: &str) -> Result<Self, Error> {
        Ok(GitStruct {
            repo: Repository::open(path)?,
        })
    }

    pub fn print(self) -> Result<(), Error> {
        let mut revwalk = self.repo.revwalk()?;

        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)?;

        for node in revwalk {
            let oid = node?;
            let commit = self.repo.find_commit(oid)?;

            let commit_time = commit.time().seconds();
            let datetime = DateTime::from_timestamp(commit_time, 0).unwrap_or_default();

            println!("{} at {} {}", oid, datetime, commit.summary().unwrap_or(""));
        }

        Ok(())
    }
}
