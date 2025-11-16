use chrono::{DateTime, Utc};
use git2::{Error, Oid, Repository, Revwalk, Signature};
use std::sync::{Arc, Mutex};

pub struct GitStruct {
    repo: Repository,
    idx: usize,
    pub vec: Arc<Mutex<Vec<Oid>>>, 
}

impl GitStruct {
    pub fn new(path: &str) -> Result<Self, Error> {
        Ok(GitStruct {
            repo: Repository::open(path)?,
            idx: 0,
            vec: Arc::new(Mutex::new(Vec::new())),
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

    pub fn get_rev_walk(&self) -> Revwalk {
        let mut revwalk = self.repo.revwalk().unwrap();

        revwalk.push_head().unwrap();
        revwalk
            .set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)
            .unwrap();

        revwalk
    }

    pub fn get_next_from_walk(&self, revwalk: &mut Revwalk) -> String {
        let next = revwalk.next();
        let oid = next.unwrap().unwrap();
        let commit = self.repo.find_commit(oid).unwrap();

        let commit_time = commit.time().seconds();
        let datetime = DateTime::from_timestamp(commit_time, 0).unwrap_or_default();
        format!("{} at {} {}", oid, datetime, commit.summary().unwrap_or(""))
    }

    pub fn get_len(&self) -> usize {
        let vec = self.vec.lock();
        vec.unwrap().len()
    }

    pub fn populate_from_walk(&mut self, revwalk: &mut Revwalk) {
         for node in revwalk {
            self.vec.lock().unwrap().push(node.unwrap());
        }
    }

    pub fn get_commit(&self) -> Oid {
        let lock = self.vec.lock().unwrap();
        lock[self.idx]
    }

    pub fn get_date(&self, oid: Oid) -> DateTime<Utc> {
        let commit: git2::Commit<'_> = self.repo.find_commit(oid).unwrap();

        let commit_time = commit.time().seconds();
        DateTime::from_timestamp(commit_time, 0).unwrap_or_default()
    }

    pub fn get_author(&self, oid: Oid) -> (Option<String>, Option<String>) {
        let commit: git2::Commit<'_> = self.repo.find_commit(oid).unwrap();

        (commit.author().name().and_then(|x| Some(x.to_string())) , commit.author().email().and_then(|x| Some(x.to_string())))
    }

    pub fn increment_idx(&mut self) {
        self.idx += 1;
        self.idx = self.idx.min(self.get_len() - 1);
    }

    pub fn decrement_idx(&mut self) {
        if self.idx == 0 {
            return;
        }
        self.idx -= 1;
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }
}