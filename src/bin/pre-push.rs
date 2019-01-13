extern crate git_hooks;

fn main() -> git_hooks::Result<()> {
    git_hooks::run_hook(git_hooks::Hook::PrePush)
}
