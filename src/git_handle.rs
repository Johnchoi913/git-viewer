use chrono::DateTime;
use git2::{Error, Repository, Revwalk, Oid};
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
}