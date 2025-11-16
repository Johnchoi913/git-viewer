mod draw_handler;
mod git_handle;

use draw_handler::DrawHandler;
use git_handle::GitStruct;
use git2::Repository;
use std::sync::Arc;
use std::thread;

fn main() -> eframe::Result<()> {
    let git_struct = match GitStruct::new(".") {
        Ok(gs) => gs,
        Err(e) => panic!("Failed to open repository: {}", e),
    };

    let vec_arc = Arc::clone(&git_struct.vec);
    let repo_path = ".".to_string();

    thread::spawn(move || {
        if let Ok(repo) = Repository::open(repo_path) {
            if let Ok(mut revwalk) = repo.revwalk() {
                revwalk.push_head().unwrap();
                revwalk
                    .set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)
                    .unwrap();


                for node in revwalk {
                    if let Ok(oid) = node {
                        vec_arc.lock().unwrap().push(oid);
                    }
                }
            }
        }
    });

    println!("Waiting for first commit to load...");
    let vec_arc2 = Arc::clone(&git_struct.vec);
    loop {
        if vec_arc2.lock().unwrap().len() > 0 {
            break; 
        }
        
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    println!("First commit loaded. Starting GUI.");

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Git Viewer",
        native_options,
        Box::new(|_| Ok(Box::new(DrawHandler::new(git_struct)))),
    )
}
