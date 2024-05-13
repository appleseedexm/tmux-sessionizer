use clap::Parser;
use error_stack::{Report, Result, ResultExt};

use tms::{
    cli::{Cli, SubCommandGiven},
    dirs::{manual_dirs, DirContainer},
    dirty_paths::DirtyUtf8Path,
    error::TmsError,
    get_single_selection,
    picker::Preview,
    repos::{find_repos, RepoContainer},
    session_exists, set_up_tmux_env, switch_to_session,
    tmux::Tmux,
    Suggestion,
};

fn main() -> Result<(), TmsError> {
    // Install debug hooks for formatting of error handling
    Report::install_debug_hook::<Suggestion>(|value, context| {
        context.push_body(format!("{value}"));
    });
    #[cfg(any(not(debug_assertions), test))]
    Report::install_debug_hook::<std::panic::Location>(|_value, _context| {});

    // Use CLAP to parse the command line arguments
    let cli_args = Cli::parse();

    let tmux = Tmux::default();

    let config = match cli_args.handle_sub_commands(&tmux)? {
        SubCommandGiven::Yes => return Ok(()),
        SubCommandGiven::No(config) => config, // continue
    };

    // Find repositories and present them with the fuzzy finder
    let repos = find_repos(
        config.search_dirs()?,
        config.excluded_dirs,
        config.display_full_path,
        config.search_submodules,
        config.recursive_submodules,
    )?;

    let manual_dirs = manual_dirs(config.manual_dirs)?.unwrap();
    let mut all_paths: Vec<String> = repos.list().clone();
    all_paths.append(&mut manual_dirs.as_strings().clone());

    let repo_name = if let Some(str) = get_single_selection(
        &all_paths,
        Preview::None,
        config.picker_colors,
        config.shortcuts,
        tmux.clone(),
    )? {
        str
    } else {
        return Ok(());
    };

    //if let Some(dirs_name) = manual_dirs.find_dir(str){
        //// todo : manual dirs
    //}

    //// todo: the found repo is still relevant down the line... 
    //let (found_repo, path) = fun_name(repos, &repo_name)?;
    //let repo_short_name = repo_short_name(config, repo_name)?;

    //switch(repo_short_name, tmux, path, found_repo)?;

    Ok(())
}

//fn fun_name(repos: impl RepoContainer, repo_name: &String) -> Result<(&git2::Repository, String), Report<TmsError>> {
    //let found_repo = repos
        //.find_repo(repo_name)
        //.expect("The internal representation of the selected repository should be present");
    //let path = if found_repo.is_bare() {
        //found_repo.path().to_string()?
    //} else {
        //found_repo
            //.workdir()
            //.expect("bare repositories should all have parent directories")
            //.canonicalize()
            //.change_context(TmsError::IoError)?
            //.to_string()?
    //};
    //Ok((found_repo, path))
//}

//fn switch(repo_short_name: String, tmux: Tmux, path: String, found_repo: &git2::Repository) -> Result<(), Report<TmsError>> {
    //if !session_exists(&repo_short_name, &tmux) {
        //tmux.new_session(Some(&repo_short_name), Some(&path));
        //set_up_tmux_env(found_repo, &repo_short_name, &tmux)?;
    //}
    //switch_to_session(&repo_short_name, &tmux);
    //Ok(())
//}

//fn repo_short_name(config: Box<tms::configs::Config>, repo_name: String) -> Result<String, Report<TmsError>> {
    //let repo_short_name = (if config.display_full_path == Some(true) {
        //std::path::PathBuf::from(&repo_name)
            //.file_name()
            //.expect("None of the paths here should terminate in `..`")
            //.to_string()?
    //} else {
        //repo_name
    //})
    //.replace('.', "_");
    //Ok(repo_short_name)
//}
