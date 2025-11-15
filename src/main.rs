mod git_handle;

use git_handle::GitStruct;

fn main() {
    let git_struct = GitStruct::new(".");
    if let Ok(git_struct) = git_struct {
        let _ = git_struct.print();
    } else {
        println!("Error printing");
    }
}
