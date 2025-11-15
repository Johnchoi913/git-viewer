use git2::{Error, Repository};

fn main() -> Result<(), Error> {
    let repo = Repository::open(".")?;

    let head = repo.head()?;

    let head_target = head.target().unwrap();

    let mut revwalk = repo.revwalk()?;

    revwalk.push(head_target)?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    for node in revwalk {
        let oid = node?;
        let commit = repo.find_commit(oid)?;
        println!("{} {}", oid, commit.summary().unwrap_or(""));
    }

    Ok(())
}
