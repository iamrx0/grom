use git2::{self, FetchOptions, IndexAddOption, PushOptions, RemoteCallbacks, Repository};

pub fn init_sync(path: String, remote: String) -> Result<(), std::io::Error> {
    let repo: Repository = Repository::init(path.clone()).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Git init failed: {e}"))
    })?;
    repo.remote_set_url("origin", remote.as_str())
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Setting remote URL failed: {e}"),
            )
        })?;
    push_initial_changes(path)?;
    Ok(())
}

pub fn push_initial_changes(path: String) -> Result<(), std::io::Error> {
    let repo: Repository = Repository::open(path).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Opening repository failed: {e}"),
        )
    })?;

    let mut index = repo.index().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Getting index failed: {e}"),
        )
    })?;
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Adding files to index failed: {e}"),
            )
        })?;
    index.write().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Writing index failed: {e}"),
        )
    })?;

    let tree_id = index.write_tree().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Writing tree failed: {e}"),
        )
    })?;
    let tree = repo.find_tree(tree_id).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Finding tree failed: {e}"),
        )
    })?;

    let signature = repo.signature().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Creating signature failed: {e}"),
        )
    })?;
    repo.commit(
        Some("refs/heads/main"),
        &signature,
        &signature,
        "Initial",
        &tree,
        &[],
    )
    .map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Committing changes failed: {e}"),
        )
    })?;

    let mut remote = repo.find_remote("origin").map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Finding remote failed: {e}"),
        )
    })?;
    let mut callbacks = git2::RemoteCallbacks::new();

    let mut auth_attempts = 0;
    let max_auth_attempts = 1;

    callbacks.credentials(move |url, username_from_url, _| {
        if auth_attempts >= max_auth_attempts {
            return Err(git2::Error::from_str(
                "Maximum authentication attempts exceeded.",
            ));
        }
        auth_attempts += 1;
        if url.starts_with("ssh://") || url.starts_with("git@") {
            match git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                Ok(cred) => Ok(cred),
                Err(_) => {
                    let username: String = cliclack::input("Enter your Git Username.")
                        .interact()
                        .unwrap();
                    let password: String = cliclack::input("Enter your Git Password.")
                        .interact()
                        .unwrap();
                    match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                        Ok(cred) => Ok(cred),
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            let username: String = cliclack::input("Enter your Git Username.")
                .interact()
                .unwrap();
            let password: String = cliclack::input("Enter your Git Password.")
                .interact()
                .unwrap();
            match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                Ok(cred) => Ok(cred),
                Err(e) => Err(e),
            }
        }
    });
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote
        .push(
            &["refs/heads/main:refs/heads/main"],
            Some(&mut push_options),
        )
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Pushing changes failed: {e}"),
            )
        })
}
pub fn push_changes(path: String, message: String) -> Result<(), std::io::Error> {
    let repo: Repository = Repository::open(path).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Opening repository failed: {e}"),
        )
    })?;

    let mut index = repo.index().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Getting index failed: {e}"),
        )
    })?;
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Adding files to index failed: {e}"),
            )
        })?;
    index.write().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Writing index failed: {e}"),
        )
    })?;

    let tree_id = index.write_tree().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Writing tree failed: {e}"),
        )
    })?;
    let tree = repo.find_tree(tree_id).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Finding tree failed: {e}"),
        )
    })?;

    let signature = repo.signature().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Creating signature failed: {e}"),
        )
    })?;
    let head = repo
        .head()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Getting HEAD failed: {e}"),
            )
        })?
        .peel_to_commit()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Peeling to commit failed: {e}"),
            )
        })?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &[&head],
    )
    .map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Committing changes failed: {e}"),
        )
    })?;

    let mut remote = repo.find_remote("origin").map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Finding remote failed: {e}"),
        )
    })?;
    let mut callbacks = git2::RemoteCallbacks::new();

    let mut auth_attempts = 0;
    let max_auth_attempts = 1;

    callbacks.credentials(move |url, username_from_url, _| {
        if auth_attempts >= max_auth_attempts {
            return Err(git2::Error::from_str(
                "Maximum authentication attempts exceeded.",
            ));
        }
        auth_attempts += 1;
        if url.starts_with("ssh://") || url.starts_with("git@") {
            match git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                Ok(cred) => Ok(cred),
                Err(_) => {
                    let username: String = cliclack::input("Enter your Git Username.")
                        .interact()
                        .unwrap();
                    let password: String = cliclack::input("Enter your Git Password.")
                        .interact()
                        .unwrap();
                    match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                        Ok(cred) => Ok(cred),
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            let username: String = cliclack::input("Enter your Git Username.")
                .interact()
                .unwrap();
            let password: String = cliclack::input("Enter your Git Password.")
                .interact()
                .unwrap();
            match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                Ok(cred) => Ok(cred),
                Err(e) => Err(e),
            }
        }
    });
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote
        .push(
            &["refs/heads/main:refs/heads/main"],
            Some(&mut push_options),
        )
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Pushing changes failed: {e}"),
            )
        })
}

pub fn pull_changes(path: String) -> Result<(), std::io::Error> {
    let repo = Repository::open(path).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Opening repository failed: {e}"),
        )
    })?;

    let mut remote = repo.find_remote("origin").map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Finding remote failed: {e}"),
        )
    })?;
    let mut remote_callbacks = RemoteCallbacks::new();

    let mut auth_attempts = 0;
    let max_auth_attempts = 1;

    remote_callbacks.credentials(move |url, username_from_url, _| {
        if auth_attempts >= max_auth_attempts {
            return Err(git2::Error::from_str(
                "Maximum authentication attempts exceeded.",
            ));
        }
        auth_attempts += 1;
        if url.starts_with("ssh://") || url.starts_with("git@") {
            match git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                Ok(cred) => Ok(cred),
                Err(_) => {
                    let username: String = cliclack::input("Enter your Git Username.")
                        .interact()
                        .unwrap();
                    let password: String = cliclack::input("Enter your Git Password.")
                        .interact()
                        .unwrap();
                    match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                        Ok(cred) => Ok(cred),
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            let username: String = cliclack::input("Enter your Git Username.")
                .interact()
                .unwrap();
            let password: String = cliclack::input("Enter your Git Password.")
                .interact()
                .unwrap();
            match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                Ok(cred) => Ok(cred),
                Err(e) => Err(e),
            }
        }
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);

    remote
        .fetch(&["main"], Some(&mut fetch_options), None)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Fetching changes failed: {e}"),
            )
        })?;

    let fetch_head = repo.find_reference("FETCH_HEAD").map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Finding FETCH_HEAD failed: {e}"),
        )
    })?;
    let fetch_commit = repo
        .reference_to_annotated_commit(&fetch_head)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Converting to annotated commit failed: {e}"),
            )
        })?;

    let analysis = repo.merge_analysis(&[&fetch_commit]).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Analyzing merge failed: {e}"),
        )
    })?;
    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", "main");
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                r.set_target(fetch_commit.id(), "Fast-Forward")
                    .map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Setting target failed: {e}"),
                        )
                    })?;
                repo.set_head(&refname).map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Setting HEAD failed: {e}"),
                    )
                })?;
                repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))
                    .map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Checking out HEAD failed: {e}"),
                        )
                    })?;
            }
            Err(_) => {
                repo.reference(&refname, fetch_commit.id(), true, "Setting reference")
                    .map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Setting reference failed: {e}"),
                        )
                    })?;
                repo.set_head(&refname).map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Setting HEAD failed: {e}"),
                    )
                })?;
                repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                    .map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Checking out HEAD failed: {e}"),
                        )
                    })?;
            }
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Non-fast-forward merge not supported.",
        ));
    }
    Ok(())
}
