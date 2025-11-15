use git2::{Error, Repository};

fn main() -> Result<(),  Error>{

    let repo = Repository::open(".")?;

    let head = repo.head()?;

    let head_target = head.target().unwrap();

    println!("{}", head_target);
    Ok(())
}
